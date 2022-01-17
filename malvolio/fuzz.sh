# note: install fuzzcheck with `cargo install cargo-fuzzcheck` first!
cargo +nightly fuzzcheck fuzz::test_emit --cargo-args "--all-features"
