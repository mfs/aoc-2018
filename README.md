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
	target/release/p1: 21ms
	target/release/p2: 2ms
	target/release/p3: 97ms
	target/release/p4: 6ms
	target/release/p5: 645ms
	target/release/p6: 137ms
	target/release/p7: 1ms
	target/release/p8: 1ms
	target/release/p9: 519ms
	target/release/p10: 8ms
	target/release/p11: 970ms
	target/release/p12: 3ms
	target/release/p13: 12ms
	target/release/p14: 534ms
	target/release/p16: 5ms
	target/release/p17: 21ms
	target/release/p18: 173ms
	target/release/p19: 163ms
	target/release/p20: 6ms

	Total time: 3334ms
