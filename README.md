
# *DISCLAIMER*

This software is still experimental. We (MOSEK) may fix errors and bugs, but provide
no guarantee on how or how quickly we do so.

# Mosek 9.2 Rust interface

- Mosek optimization software: [https://mosek.com](https://mosek.com)
- Rust language: [https://www.rust-lang.com](https://www.rust-lang.com)

API reference is included under `doc/` and is fairly complete, but currently contains a lot of dead links.

The package should work on

- Linux x86_64
- Linux aarch64 (RaspberryPi 4)
- Windows x86_64
- Mac OSX x86_64


Building the API requires the MOSEK library. The exact version is specified in [./MOSEKVERSION](MOSEKVERSION)
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
export MOSEK_BINDIR_93=$HOME/local/mosek/9.2/tools/platform/linux64x86/bin
cargo build
```

## Examples

Examples are located under `examples/`

To compile examples, run

```
cargo build --examples
```
