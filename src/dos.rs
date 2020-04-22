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

pub fn get_keyboard_input() -> u8 {
    unsafe {
        rusty_asm! {
            let code: out("{ax}");
            asm("volatile", "intel") {
                "mov  ah,01h
                 int  16h
                 jz   empty
                 mov  ah,00h
                 int  16h
                 mov  al,ah

                 xor  ah,ah
                 jmp  done
                 
                 empty:
                    xor ax,ax
                 done:"
            }
            return code;
        }
    }
}

pub fn set_video_mode(m: u8) {
    unsafe {
        rusty_asm! {
            let mode: in("{al}") = m as u8;
            clobber("ax");
            asm("volatile", "intel") {
                "mov ah, 0
                int 10h"
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