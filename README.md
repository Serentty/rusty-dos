Rusty DOS
=========

A Rust skeleton for an MS-DOS program for IBM compatibles and the PC-98, including some PC-98-specific functionality. It requires at least a 386, as this is LLVM's minimum supported x86 target. Right now it's in real mode (albeit wastefully using 32-bit near pointers that still only have a 64 KiB range). My plan is currently to start using unreal mode if possible, which might allow the requirements to be lowered to a 286 in the future, if compiler support gets there. I had considered switching it to use protected mode and DJGPP, but unreal mode has a sort of charm to it as a weird hack, as well as being closer to real mode for that classic, bare-metal feel.

Unicode
-------

Strings are all Unicode-enabled, encoded using UTF-8, just as they usually are in Rust. When writing strings, they get converted to codepage 437 at the last minute, but this does not require any allocation as it done character-by-character instead of converting the whole string. This unfortunately means that any decomposed representation will not work, but to be honest, everyone uses the precomposed versions of all of the characters in codepage 437 anyway.

The advantage of this approach is that it means that using your existing Rust code should be seamless. There is no need to work with byte strings instead. ASCII strings should be really fast, while the additional characters available in codepage 437 require only a small table lookup which is practically instanteous on a 386. Unsupported characters will show up as tofu instead of mojibake, which I find more user-friendly.

Building
--------

Thanks to the build-std feature, you should only need a vanilla Rust toolchain now. I build it like this, but you can tune it for your needs.

```
RUSTFLAGS="-C opt-level=z" cargo build --release --target dos.json
```

It seems that this won't build properly with MSVC's linker. Using GCC instead should fix this.
