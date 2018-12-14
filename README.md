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
	target/release/p1: 23ms
	target/release/p2: 6ms
	target/release/p3: 91ms
	target/release/p4: 17ms
	target/release/p5: 720ms
	target/release/p6: 146ms
	target/release/p7: 5ms
	target/release/p8: 6ms
	target/release/p9: 527ms
	target/release/p10: 21ms
	target/release/p11: 973ms
	target/release/p12: 3ms
	target/release/p13: 11ms
	target/release/p14: 554ms

	Total time: 3109ms
