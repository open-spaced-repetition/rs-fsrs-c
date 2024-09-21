#!/bin/sh
set -eux
cargo build
find target -name "librs_fsrs_c*" -type f -print -exec gcc examples/basic.c target/debug/{} \;
./a.out
