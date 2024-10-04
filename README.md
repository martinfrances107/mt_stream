# MT_STREAM

This is just a scratch pad where I explore patterns centered around
making [rust_g3_geo](https://crates.io/crates/d3_geo_rs) multi-threaded.

In rust_d3_geo each stage of the 6 stage pipeline will be a new thread - Message(Enum) is passes via a "std::sync::mpscc"

Depends on [anyhow](https://crates.io/crates/anyhow) otherwise the two possible return types become complex.
