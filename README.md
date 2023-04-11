# Mosek 10.1 Rust interface

- Mosek optimization software: [https://mosek.com/]
- Github repository for this package: [https://github.com/MOSEK/mosek.rust]
- Rust language: [https://www.rust-lang.com]
- Pre-packaged crates are available from [https://crates.io]

The package should work on

- Linux x86_64
- Linux aarch64 (RaspberryPi 4, Amazon Graviton 2 and others)
- Windows x86_64
- Mac OSX x86_64
- Mac OSX aarch64

Building the API requires the MOSEK library.
- If the environment variable `MOSEK_BINDIR_101` is defined it is expected to
  point the the directory containing the MOSEK binaries,
- otherwise if the environment variable `MOSEK_INST_BASE`, the build
  script will look for mosek `$MOSEK_INST_BASE/mosek`,
- otherwise  the build script will look for MOSEK in `$HOME/mosek`.
- If none of the above produces a MOSEK distro, on linux and OSX it will
  attempt to download and unpack the latest MOSEK distro (this requires
  external tools `curl`, `tar` and `bzip2`). On Windows it will fail.

For example
```
export MOSEK_INST_BASE=$HOME/local
cargo build
```
or
```
export MOSEK_BINDIR_101=$HOME/local/mosek/10.1/tools/platform/linux64x86/bin
cargo build
```

## External dependencies
The MOSEK Rust API currently depends only on `libc` and `itertools`, and the
MOSEK library.

## Documentation

```
cargo doc
```

will build the simple API documentation for all
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
mosek = "10.1"
```

# Why Use Rust with Mosek?

Rust has many advantages over other languages supported directly by MOSEK. For
data wrangling it is faster than Python, Java or .NET, and it is significantly
safer than C or C++. When building non-trivial models, the time it takes to
form the input data for a problem may become non-trivial as well. When
efficiency is critical, the traditional language of choice would have been C or C++, 
but now Rust provides a much safer alternative. 

Compared to Java and .NET Rust is in many cases somewhat faster when e.g.
building complex constraint matrixes.

Finally, it looks good. Rust language facilities allow us to write many array
operations very compactly, yielding concise and readable model code.

