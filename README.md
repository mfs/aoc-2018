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
	target/release/p1: 18ms
	target/release/p2: 2ms
	target/release/p3: 97ms
	target/release/p4: 10ms
	target/release/p5: 758ms
	target/release/p6: 148ms
	target/release/p7: 1ms
	target/release/p8: 2ms
	target/release/p9: 553ms

	Total time: 1592ms
