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
	target/release/p2: 7ms
	target/release/p3: 111ms
	target/release/p4: 19ms
	target/release/p5: 734ms
	target/release/p6: 153ms
	target/release/p7: 8ms
	target/release/p8: 7ms
	target/release/p9: 566ms
	target/release/p10: 21ms
	target/release/p11: 1071ms
	target/release/p12: 11ms
	target/release/p13: 16ms
	target/release/p14: 607ms
	target/release/p16: 22ms
	target/release/p17: 38ms
	target/release/p18: 191ms
	target/release/p19: 179ms
	target/release/p20: 12ms
	target/release/p21: 67449ms <- well this is awkward

	Total time: 71256ms
