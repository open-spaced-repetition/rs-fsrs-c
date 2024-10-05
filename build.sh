#!/usr/bin/env bash
set -eux
cargo build
BASEDIR="$( cd "$( dirname "$0" )" && pwd )"
gcc -o a.out examples/basic.c -L${BASEDIR}/target/debug/ -lrs_fsrs_c -I${BASEDIR}/include/
env LD_LIBRARY_PATH=${BASEDIR}/target/debug/ ./a.out
