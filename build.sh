#!/bin/sh
set -eux
cargo build
find target -name "librs_fsrs_c.so" -not -path "*/deps/*" -type f -print -exec gcc examples/basic.c {} \;
./a.out
