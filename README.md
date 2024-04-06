# WC Clone

> Clone of UNIX command for counting bytes, lines, words of a text file. Made for educational purposes.

## Build

`cargo build`

## Run e2e tests (after `build`)

`cargo test`

## Use (without explicit building)

- Get help: `cargo run -- --help`
- Count bytes: `cargo run -- -c tests/test.txt`
- Count lines: `cargo run -- -l tests/test.txt`
- Count words: `cargo run -- -w tests/test.txt`
- Count all: `cargo run -- tests/test.txt`
- Count all using pipe: `cat tests/test.txt | cargo run`
