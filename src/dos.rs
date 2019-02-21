use rusty_asm::rusty_asm;

pub fn print(s: *const u8) {
    unsafe {
        rusty_asm! {
            let ptr: in("{dx}") = s;
            clobber("ah");
            clobber("al");
            asm("volatile", "intel") {
                "mov ah, 09h
                int 21h"
            }
        }
    }
}

pub fn print_character(c: u8) {
    unsafe {
        rusty_asm! {
            let byte: in("{dl}") = c;
            clobber("ah");
            clobber("al");
            asm("volatile", "intel") {
                "mov ah, 02h
                int 21h"
            }
        }
    }
}

pub fn exit() {
    unsafe {
        rusty_asm! {
            asm("volatile", "intel") {
                "mov ah, 4Ch
                int 21h"
            }
        }
    }
}