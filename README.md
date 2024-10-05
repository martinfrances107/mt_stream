# MT_STREAM

This is just a scratch pad where I explore patterns centered around
making [rust_g3_geo](https://crates.io/crates/d3_geo_rs) multi-threaded.

In rust_d3_geo each stage of the 6 stage pipeline will be a new thread - Message(Enum) is passes via a "std::sync::mpscc"

Each thread loop over a input stage

Questions Remaining.

How to I terminate the thread and signal that the stream is complete.
So that I can acces the state of the endpoint via Result?
