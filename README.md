# Advent of Code 2018

https://adventofcode.com/2018

My solutions in Rust. Concentrating on writing idiomatic Rust and using
efficient algorithms.

## Executing

To run a particular puzzle use the `--bin` parameter to cargo

	cargo run --release --bin p1

## Benchmarks

To calculate benchmark times run `./benchmark.sh` from the top directory. This
will build release versions of all puzzles and then execute them collecting
timing information.

	Times
	----------------------------------------
	target/release/p1: 19ms
	target/release/p2: 3ms
	target/release/p3: 94ms
	target/release/p4: 8ms
	target/release/p5: 757ms
	target/release/p6: 143ms
	target/release/p7: 3ms
	target/release/p8: 2ms

	Total time: 1032ms

