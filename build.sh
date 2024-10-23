#!/usr/bin/env bash
set -eux -o pipefail

cargo build

gcc \
    -o a.out \
    -L$PWD/target/debug/ \
    -lrs_fsrs_c \
    -I$PWD/include/ \
    -Wall -Wextra -pedantic -fprofile-arcs -ftest-coverage \
    examples/basic.c
env LD_LIBRARY_PATH=$PWD/target/debug/ ./a.out
gcov examples/basic.c
cat examples/basic.c.gcov
