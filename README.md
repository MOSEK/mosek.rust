# *DISCLAIMER* 

This software is experimental. We (MOSEK) may fix errors and bugs, but provide
no guarantee on how quickly or how we do so. We may at any point decide to
terminate support for the code.

# Mosek Rust interface

Mosek:
    https://mosek.com/
Rust:
     https://www.rust-lang.com

API reference is included under `doc/` and is fairly complete, but currently contains a lot of dead links.

Building requires MOSEK to be installed. By default the build script will look
for MOSEK in `$HOME/mosek`, but if the location is different, set the
environment variable `MOSEK_INST_BASE` to point to the directory where MOSEK
is installed before building.

To build, use
```
cargo build
```



## Examples

Examples are located under `examples/`

To compile examples, run `cargo test`

