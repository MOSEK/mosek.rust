# Mosek 12.0 Rust interface

- Mosek optimization software: https://mosek.com/
- Github repository for this package: https://github.com/MOSEK/mosek.rust
- Rust language: https://www.rust-lang.com
- Pre-packaged crates are available from https://crates.io/crates/MOSEK

The package should work on

- Linux x86_64
- Linux aarch64 (RaspberryPi 4+, Amazon Graviton 2+ and others)
- Windows x86_64
- Mac OSX aarch64

The library is designed to have minimal external dependencies, currently:
- `libc` used for foreign function calls,
- `itertools` 
- `cc` (build only) for compiling included C files


## Building

The package can be used in two modes: If the feature `dynamic` is disabled (default), a MOSEK installation must be
available at build time, otherwise MOSEK is only required at runtime.

### Feature `dynamic` disabled (default)

Building required the MOSEK library. By default, the build script search for MOSEK in the following locations:
- If the environment defines `MOSEKRS_FORCE_DOWNLOAD=TRUE`, the build script will attempt to download and install MOSEK
  in the build directory. This requires external tools `curl`, `tar` and `bzip2`, and on Windows it will fail. Otherwise
- If the MOSEK command line tool is on the `PATH`, this will be used to locate the MOSEK library. This will work if
  either the MOSEK distro `bin` directory is in the `PATH`, or if a symlink to the `mosek` binary is in the path.
- Otherwise following locations are search:
  - `$HOME/mosek` (linux, osx)
  - `$USERPROFILE/mosek` (windows)
  - `$HOME/Applications/mosek` (osx)
  - `$HOME/.local/mosek` (osx, linux)
  - `$LOCALAPPDATA/mosek` (windows)
- If MOSEK is not found, the build script will not specify a link path, and building will most likely fail.

At runtime, the MOSEK library must be present in the system loader path.

### Feature `dynamic` enabled

Building does not require MOSEK. At runtime MOSEK is loaded by a call to
`mosek::initialize(paths:Option<&[&std::path::Path>])`. If `paths` is `None`, the MOSEK library must be present in the
system loader path, otherwise the given paths are searched for the MOSEK library.

# Documentation

```
cargo doc
```

will build the simple API documentation for all
functions, objects and constants. For a more complete documentation,
see <https://docs.mosek.com/latest/capi/index.html>.

# Examples

Examples are located under `examples/`

To compile examples, run

```
cargo build --examples
```

To run example binaries it is necessary to add the path to the MOSEK
library to the `LD_LIBRARY_PATH` (linux), `DYLD_LIBRARY_PATH` (OS X)
or `PATH` (Windows) environment variable.

# Using MOSEK in another project

To use MOSEK from another Rust project, add "mosek" to the dependencies.
Normally, it will be a good idea to specify an exact major and minor version
for the dependency since there is no guarantee that the MOSEK API will not
change between minor versions (though usually it will not change much).

For example, add to your `Cargo.toml`:
```
[dependencies]
mosek = "12.0"
```

When running a project that uses `mosek`, the mosek library must be in the
library search path (`PATH` for Windows, `LD_LIBRARY_PATH` for linux,
`DYLD_LIBRARY_PATH` for OS X).


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
