#!/bin/bash

set -e

cargo build --release

target/release/benchmark | tee benchmark.log
