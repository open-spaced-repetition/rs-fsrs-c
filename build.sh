#!/bin/sh
set -eux
cargo build
find target \
  -name "librs_fsrs_c.so" \
  -not -path "*/deps/*" \
  -type f -print \
  -exec env LD_LIBRARY_PATH=target/debug/librs_fsrs_c.so gcc examples/basic.c {} \;
./a.out
