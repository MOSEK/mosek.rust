# Mosek 10.1 Rust interface

- Mosek optimization software: [https://mosek.com/]
- Rust language: [https://www.rust-lang.com]
- Pre-packaged crates are available from [https://crates.io]

The package should work on

- Linux x86_64
- Linux aarch64 (RaspberryPi 4, Amazon Graviton 2 and others)
- Windows x86_64
- Mac OSX x86_64
- Mac OSX aarch64

Building the API requires the MOSEK library.
- If the environment variable `MOSEK_BINDIR_100` is defined it is expected to
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
export MOSEK_BINDIR_101=$HOME/local/mosek/10.0/tools/platform/linux64x86/bin
cargo build
```

## External dependencies
The MOSEK Rust API currently depends only on `libc` and `itertools`, and the
MOSEK library.

## Documentation

```cargo doc``` will build the simple API documentation for all
functions, objects and constants. For a more complete documentation,
see <https://docs.mosek.com/latest/capi/index.html>.

## Examples

Examples are located under `examples/`

To compile examples, run

```
cargo build --examples
```

To run example binaries it is necessary to add the path to the MOSEK
library to the `LD_LIBRARY_PATH` (linux), `DYLD_LIBRARY_PATH` (OS X)
or `PATH` (Windows) environment variable.

## Using MOSEK in another project

To use MOSEK from another Rust project, add "mosek" to the dependencies.
Normally, it will be a good idea to specify an exact major and minor version
for the dependency since there is no guarantee that the MOSEK API will not
change between minor versions (though usually it will not change much).

For example, add to your `Cargo.toml`:
```
[dependencies]
mosek = "10.0"
```
