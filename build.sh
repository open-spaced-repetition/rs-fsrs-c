#!/bin/sh
set -eux
cargo build
find target -name "librs_fsrs_c.so" -type f -print -exec gcc examples/basic.c {} \;
./a.out
