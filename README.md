Rusty DOS
=========

A Rust skeleton for an MS-DOS program for IBM compatibles and the PC-98, including some PC-98-specific functionality. It requires at least a 386, as this is LLVM's minimum supported x86 target. Right now it's in real mode (albeit wastefully using 32-bit near pointers that still only have a 64 KiB range). My plan is currently to start using unreal mode if possible, which might allow the requirements to be lowered to a 286 in the future, if compiler support gets there. I had considered switching it to use protected mode and DJGPP, but unreal mode has a sort of charm to it as a weird hack, as well as being closer to real mode for that classic, bare-metal feel.

Building
--------

Thanks to the build-std feature, you should only need a vanilla Rust toolchain now. I build it like this, but you can tune it for your needs.

```
RUSTFLAGS="-C opt-level=z" cargo build --release --target dos.json
```

It seems that this won't build properly with MSVC's linker. Using GCC instead should fix this.
