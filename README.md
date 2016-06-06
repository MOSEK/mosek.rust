# *DISCLAIMER*

This software is experimental. We (MOSEK) may fix errors and bugs, but provide
no guarantee on how or how quickly we do so.

The API has been tested on 64bit Linux, but may work on other posix platforms as well.

# Mosek Rust interface

Mosek:

    https://mosek.com/

Rust:

    https://www.rust-lang.com


API reference is included under `doc/` and is fairly complete, but currently contains a lot of dead links.

Building the API requires the MOSEK 8.0 library.
- By default the build script will look for MOSEK in `$HOME/mosek`.
- If `MOSEK_INST_BASE` is defined, this will override `$HOME`.
- If `MOSEK_8_BINDIR` is defined, this overrides all others, and it is
  expected to point the the directory containing the MOSEK binaries.

For example
```
export MOSEK_INST_BASE=$HOME/local
cargo build
```
or
```
export MOSEK_8_BINDIR=$HOME/local/mosek/8/tools/platform/linux64x86/bin
cargo build
```

## Examples

Examples are located under `examples/`

To compile examples, run

```
cargo test
```

