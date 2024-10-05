// use anyhow::Context;
// use anyhow::Result;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::RecvError;
use std::sync::mpsc::SendError;
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

enum ChannelError {
    Rx(RecvError),
    Tx(SendError<Message<f64>>),
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
    let stage1: JoinHandle<ChannelError> = thread::spawn(move || {
        // The thread takes ownership over `thread_tx`
        // Each thread queues a message in the channel
        let a;
        loop {
            let result = rx1.recv();
            a = match result {
                Ok(message) => {
                    let res_tx = match message {
                        Message::EndPoint => tx2.send(message),
                        Message::PolygonStart => tx2.send(message),
                        Message::LineStart => tx2.send(message),
                        Message::Point(_) => {
                            println!("stage 1 point entry()");
                            tx2.send(message)
                        }
                        Message::LineEnd => tx2.send(message),
                        Message::PolygonEnd => tx2.send(message),
                    };
                    match res_tx {
                        Ok(_) => {
                            continue;
                        }
                        Err(e) => ChannelError::Tx(e),
                    }
                }
                Err(e) => ChannelError::Rx(e),
            };

            match a {
                ChannelError::Rx(_) => break,
                ChannelError::Tx(_) => break,
            }
        }
        a
    });
    handles.push(stage1);

    let stage2: JoinHandle<ChannelError> = thread::spawn(move || {
        // The thread takes ownership over `thread_tx`
        // Each thread queues a message in the channel
        let a;
        loop {
            let result = rx2.recv();
            a = match result {
                Ok(message) => {
                    let res_tx = match message {
                        Message::EndPoint => tx3.send(message),
                        Message::PolygonStart => tx3.send(message),
                        Message::LineStart => tx3.send(message),
                        Message::Point(_) => {
                            println!("stage 2 point entry()");
                            tx3.send(message)
                        }
                        Message::LineEnd => tx3.send(message),
                        Message::PolygonEnd => tx3.send(message),
                    };
                    match res_tx {
                        Ok(_) => {
                            continue;
                        }
                        Err(e) => ChannelError::Tx(e),
                    }
                }
                Err(e) => ChannelError::Rx(e),
            };

            match a {
                ChannelError::Rx(_) => break,
                ChannelError::Tx(_) => break,
            }
        }
        a
    });
    handles.push(stage2);

    let stage3: JoinHandle<ChannelError> = thread::spawn(move || {
        // The thread takes ownership over `thread_tx`
        // Each thread queues a message in the channel
        let a;
        loop {
            let result = rx3.recv();
            a = match result {
                Ok(message) => {
                    match message {
                        Message::EndPoint => println!("Stage 3 Endpoint"),
                        Message::PolygonStart => println!("Stage 3 PolygonStart"),
                        Message::LineStart => println!("Stage 3 LineStart"),
                        Message::Point((c, m)) => {
                            println!("stage 3 Point {c:#?} m{m:#?}");
                        }
                        Message::LineEnd => println!("Stage 3 LineEnd"),
                        Message::PolygonEnd => println!("Stage 3 PolygonEnd"),
                    };
                    continue;
                }
                Err(e) => ChannelError::Rx(e),
            };

            match a {
                ChannelError::Rx(_) => break,
                ChannelError::Tx(_) => break,
            }
        }
        a
    });
    handles.push(stage3);

    {
        tx1.send(Message::LineStart).expect("step 1 fail");
        tx1.send(Message::Point((Coord { x: 1., y: 2. }, 25u8)))
            .expect("step 2 failed");
        tx1.send(Message::Point((Coord { x: 10., y: 20. }, 250u8)))
            .expect("step 2 failed");
        tx1.send(Message::LineEnd).expect("step 4 failed");
    }
    // Wait for all the stages to complete.
    for h in handles {
        h.join().unwrap();
    }
}
