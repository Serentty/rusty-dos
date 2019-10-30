#![feature(proc_macro_hygiene)]
#![feature(asm)]
#![no_main]
#![no_std]

mod dos;
mod panic;
mod io;
mod port;
mod opn;

#[no_mangle]
pub extern "C" fn start() {
    print!(b"Hello, world!");
}
