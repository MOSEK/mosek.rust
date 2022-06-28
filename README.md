# *DISCLAIMER*

This software is still experimental. We (MOSEK) may fix errors and bugs, but provide
no guarantee on how or how quickly we do so.


# Mosek 10.0 Rust interface

- Mosek optimization software: https://mosek.com/
- Rust language: https://www.rust-lang.com

The package should work on

- Linux x86_64
- Linux aarch64 (RaspberryPi 4, Amazon Graviton 2 and others)
- Windows x86_64
- Mac OSX x86_64
- Mac OSX aarch64

API reference is included under `doc/` and is fairly complete, but currently contains a lot of dead links.

Building the API requires the MOSEK library.
- If the environment variable `MOSEK_BINDIR_XY` (where `X` and `Y` are
  the MOSEK major an minor versions) is defined it is expected to
  point the the directory containing the MOSEK binaries,
- otherwise if the environment variable `MOSEK_INST_BASE`, the build
  script will look for mosek `$MOSEK_INST_BASE/mosek`,
- otherwise  the build script will look for MOSEK in `$HOME/mosek`.

For example
```
export MOSEK_INST_BASE=$HOME/local
cargo build
```
or
```
export MOSEK_BINDIR_100=$HOME/local/mosek/10.0/tools/platform/linux64x86/bin
cargo build
```

## Examples

Examples are located under `examples/`

To compile examples, run

```
cargo build --examples
```

# MOSEK / Mosek.rs versions

- Mosek.rs 0.1 -> MOSEK 9.2
- Mosek.rs 0.2 -> MOSEK 9.3

From Mosek 10.0 the major/minor version of Mosek Rust API will follow Mosek.

