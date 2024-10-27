CARGO_PROFILE_RELEASE_DEBUG=true cargo build --release
perf record -F 997 -e cache-misses -e cycles -g target/release/pijersi-rs
