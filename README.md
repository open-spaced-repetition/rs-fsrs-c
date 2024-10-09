# C/C++ binding for rs-fsrs

a PoC

## Example

see [example](./examples/basic.c)

## Development

see [build.sh](./build.sh)

## Name conversion

struct name: `fsrs_` + struct name in rust

enum name: `fsrs_` + enum name in rust

struct method name: `fsrs_` + struct name + `_` + method name in rust
