use anyhow::Context;
use anyhow::Result;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;

#[derive(Default, Debug)]
struct Coord<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
enum Message<T> {
    EndPoint,
    PolygonStart,
    LineStart,
    Point((Coord<T>, u8)),
    LineEnd,
    PolygonEnd,
}

fn main() {
    println!("Hello, world!");

    // Construct pipeline.
    // Channels have two endpoints: the `Sender<T>` and the `Receiver<T>`,
    // where `T` is the type of the message to be transferred
    // (type annotation is superfluous)
    // input to stage 1. rx consumed in stage 1
    let (tx1, rx1): (Sender<Message<f64>>, Receiver<Message<f64>>) = mpsc::channel();
    // input to stage 2. ex consumed in stage 2
    let (tx2, rx2): (Sender<Message<f64>>, Receiver<Message<f64>>) = mpsc::channel();
    // input to stage 3. ex consumed in stage 3
    let (tx3, rx3): (Sender<Message<f64>>, Receiver<Message<f64>>) = mpsc::channel();

    let mut handles = vec![];
    let stage1: JoinHandle<Result<(), _>> = thread::spawn(move || {
        // The thread takes ownership over `thread_tx`
        // Each thread queues a message in the channel

        let message = rx1.recv().context("stage 1 rx error")?;
        match message {
            message => match message {
                Message::EndPoint => tx2.send(message).context("s1"),
                Message::PolygonStart => tx2.send(message).context("s1"),
                Message::LineStart => tx2.send(message).context("s1"),
                Message::Point(_) => {
                    println!("stage 1 point entry()");
                    tx2.send(message).context("s1")
                }
                Message::LineEnd => tx2.send(message).context("s1"),
                Message::PolygonEnd => tx2.send(message).context("s1"),
            },
        }
    });
    handles.push(stage1);

    let stage2: JoinHandle<Result<(), _>> = thread::spawn(move || {
        // The thread takes ownership over `thread_tx`
        // Each thread queues a message in the channel
        let message = rx2.recv().context("stage 2 error")?;
        match message {
            Message::EndPoint => tx3.send(message).context("a"),
            Message::PolygonStart => tx3.send(message).context("a"),
            Message::LineStart => tx3.send(message).context("a"),
            Message::Point(_) => {
                println!("stage 2 point entry()");
                tx3.send(message).context("a")
            }
            Message::LineEnd => tx3.send(message).context("a"),
            Message::PolygonEnd => tx3.send(message).context("a"),
        }
    });
    handles.push(stage2);

    let stage3 = thread::spawn(move || {
        // The thread takes ownership over `thread_tx`
        // Each thread queues a message in the channel
        match rx3.recv() {
            Ok(message) => match message {
                Message::EndPoint => {
                    println!("stage 3 Endpoint()");
                }
                Message::PolygonStart => println!("PolygonStart"),
                Message::LineStart => println!("LineStart"),
                Message::Point((c, m)) => {
                    println!("stage 3 point entry() {:#?} {}", c, m);
                }
                Message::LineEnd => println!("LineEnd"),
                Message::PolygonEnd => println!("PolygonEnd"),
            },
            Err(_) => {
                println!("Stage 3 is closing");
                panic!();
            }
        }
        Ok(())
    });
    handles.push(stage3);

    tx1.send(Message::Point((Coord { x: 1., y: 2. }, 25u8)))
        .expect("initial submission");

    // Wait for all the stages to complete.
    for h in handles {
        h.join().unwrap().expect("Failed join stages");
    }
}
