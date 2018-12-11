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
	target/release/p1: 27ms
	target/release/p2: 2ms
	target/release/p3: 103ms
	target/release/p4: 6ms
	target/release/p5: 722ms
	target/release/p6: 140ms
	target/release/p7: 2ms
	target/release/p8: 2ms
	target/release/p9: 515ms
	target/release/p10: 8ms
	target/release/p11: 985ms

	Total time: 2516ms
