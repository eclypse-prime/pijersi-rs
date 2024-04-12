# pijersi-engine

This project is a Rust implementation of a Pijersi game engine. It can be used standalone (using the [UGI protocol](https://github.com/arthur-liu-lsh/pijersi-engine/blob/main/ugi.md)) and will also provide bindings for use in C#/Unity and Python projects. (to do)

To learn more about Pijersi: [GitHub](https://github.com/LucasBorboleta/pijersi).
The Unity project can be found here: [GitHub](https://github.com/arthur-liu-lsh/pijersi-unity).

The project is adapted from the C++ implementation: [GitHub](https://github.com/arthur-liu-lsh/pijersi-engine).

## Requirements

* Rust 1.79.0+ (nightly)

## Instructions

TBD

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