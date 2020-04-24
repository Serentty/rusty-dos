Rusty DOS
=========

A Rust skeleton for an MS-DOS program for IBM compatibles and the PC-98, including some PC-98-specific functionality. It requires at least a 386, as this is LLVM's minimum supported x86 target. Right now it's in real mode (albeit wastefully using 32-bit near pointers that still only have a 64 KiB range). It also has issues when using certain language features due to the inability to build ``core`` using the same relocation model as the program itself. I'm working on switching it to use protected mode and DJGPP, and once that's working properly, it will still only require a 386, but will use full 32-bit pointers with a flat address space (which is good because Rust has no concept of segmentation), and also have access to DJGPP's standard library. When I've ironed out the remaining issues with it, I'll put it on the master branch here, and move the current real mode version to another branch.

Building
--------

You can compile this with the following command. 
It requires the nightly toolchain, which can be installed with [rustup](https://rustup.rs/) and [cargo-xbuild](https://github.com/rust-osdev/cargo-xbuild).

```
RUSTFLAGS="-C opt-level=z -C relocation-model=static" cargo xbuild --release --target dos.json
```

It seems that this won't build properly with MSVC's linker. Using GCC instead should fix this.
