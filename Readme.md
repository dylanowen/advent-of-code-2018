# Advent of Code 2018

https://adventofcode.com/2018

## WASM Visualizations
https://dylanowen.github.io/advent-of-code-2018/

## Running

`cargo run --bin <day>`

#### With Backtrace

`RUST_BACKTRACE=1 cargo run --bin <day>`

## Wasm

### Install
This needs a version greater than `0.5.1`
`cargo install --git https://github.com/rustwasm/wasm-pack.git` 

or if a later version has been published

`cargo install wasm-pack`

### Build
`./<day>/build.sh`