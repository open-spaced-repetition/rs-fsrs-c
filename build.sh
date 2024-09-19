#!/bin/sh
set -eux
cargo build
gcc examples/basic.c target/debug/librs_fsrs_c.so
./a.out

