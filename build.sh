#!/usr/bin/env bash
set -eux -o pipefail

cargo build
mv examples/basic.c .
gcc \
    -o a.out \
    -L$PWD/target/debug/ \
    -lrs_fsrs_c \
    -I$PWD/include/ \
    -Wall -Wextra -pedantic -fprofile-arcs -ftest-coverage \
    basic.c
env LD_LIBRARY_PATH=$PWD/target/debug/ ./a.out
gcov basic.c
cat basic.c.gcov
