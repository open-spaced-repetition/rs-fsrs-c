#!/usr/bin/env bash
set -eux -o pipefail

cargo build
BASEDIR="$( cd "$( dirname "$0" )" && pwd )"
gcc -o a.out examples/basic.c -L${BASEDIR}/target/debug/ -lrs_fsrs_c -I${BASEDIR}/include/ -Wall -Wextra -pedantic  -fprofile-arcs -ftest-coverage
if [ "$(expr substr $(uname -s) 1 10)" == "MINGW64_NT" ]; then
    cp target/debug/rs_fsrs_c.dll .
    ./a.out
    gcov *.gcno *.gcda
    cat basic.c.gcov
else
    LD_LIBRARY_PATH=${BASEDIR}/target/debug/ ./a.out
    LD_LIBRARY_PATH=${BASEDIR}/target/debug/ gcov *.gcno *.gcda
    cat *.gcov
fi
