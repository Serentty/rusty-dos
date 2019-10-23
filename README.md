Rusty DOS
=========

A Rust skeleton for an MS-DOS program for IBM compatibles and the PC-98, including some PC-98-specific functionality.

Building
--------

You can compile this with the following command. 
It requires the nightly toolchain, which can be installed with [rustup](https://rustup.rs/) and [cargo-xbuild](https://github.com/rust-osdev/cargo-xbuild).

```
RUSTFLAGS="-C opt-level=z -C relocation-model=static" cargo xbuild --release --target dos.json
```
