# glider

[![Rust](https://github.com/ahamez/glider/actions/workflows/rust.yml/badge.svg)](https://github.com/ahamez/glider/actions/workflows/rust.yml)

A game of life in Rust.

## Run

```sh
cargo build --release
./target/release/glider ./glider.rle
```

It can read patterns in [RLE](https://conwaylife.com/wiki/Run_Length_Encoded) format.

## Build on macOS

If SDL is installed with homebrew:

```sh
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"

```
