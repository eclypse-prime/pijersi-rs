# pijersi-rs

This project is a Rust implementation of a Pijersi game engine. It can be used standalone (using the [UGI protocol](https://github.com/eclypse-prime/pijersi-rs/blob/main/UGI.md)) and will also provide bindings for use in C#/Unity and Python projects. (to do)

[![Static Badge](https://img.shields.io/badge/documentation-github.io-blue)](https://eclypse-prime.github.io/pijersi-rs/)
[![GitHub Release](https://img.shields.io/github/v/release/eclypse-prime/pijersi-rs)]()
[![CI/CD](https://github.com/eclypse-prime/pijersi-rs/actions/workflows/pijersi-rs-ci-cd.yml/badge.svg)](https://github.com/eclypse-prime/pijersi-rs/actions/workflows/pijersi-rs-ci-cd.yml)

To learn more about Pijersi: [GitHub](https://github.com/LucasBorboleta/pijersi).
The Unity project can be found here: [GitHub](https://github.com/arthur-liu-lsh/pijersi-unity).

The project is adapted from the C++ implementation: [GitHub](https://github.com/arthur-liu-lsh/pijersi-engine).

## Requirements

* Rust 1.80.0+

## Instructions

### Build (native Linux)

* Make sure [rustup](https://rust-lang.github.io/rustup/installation/index.html) is installed
* Clone the repo
* Install the hooks by running `install-hooks.sh`
* Download the openings by running `update-openings.sh`
* Run `cargo build --release`
* The executable is in `target/release/pijersi-rs`

### Cross compile build (for Windows)

* Make sure [rustup](https://rust-lang.github.io/rustup/installation/index.html) is installed
* Install the windows gnu target: `rustup target add x86_64-pc-windows-gnu`
* Clone the repo
* Install the hooks by running `install-hooks.sh`
* Download the openings by running `update-openings.sh`
* Run `cargo build --release --target x86_64-pc-windows-gnu`
* The executable is in `target/x86_64-pc-windows-gnu/release/pijersi-rs.exe`

## Useful data

### Perft results

Ran on 32 threads Ryzen 9 7945HX.

| Depth | Leaf nodes         | Time (ms) |
|-------|--------------------|-----------|
| 0     | 1                  | ?         |
| 1     | 186                | ?    |
| 2     | 34,054             | ?     |
| 3     | 6,410,472          | ?     |
| 4     | 1,181,445,032      | ?   |
| 5     | 220,561,140,835    | ?  |
| 6     | 40,310,812,241,663 | ?  |