use core::arch::asm;

pub fn print(s: *const u8) {
    unsafe {
        asm!(
            "int 21h",
            inout("ax") 0x0900 => _,
            in("dx") s,
        );
    }
}

pub fn print_character(c: u8) {
    unsafe {
        asm!(
            "int 21h",
            inout("ax") 0x0200 => _,
            in("dl") c,
        );
    }
}

pub fn get_keyboard_input() -> u8 {
    let code;
    unsafe {
        asm!(
            "mov ah, 01h",
            "int 16h",
            "jz 1f",
            "mov ah, 00h",
            "int 16h",
            "mov al, ah",
            "xor ah, ah",
            "jmp 2f",
            "1:",
            "xor ax, ax",
            "2:",
            out("al") code,
        );
    }
    code
}

pub fn set_video_mode(mode: u8) {
    unsafe {
        asm!(
            "int 10h",
            inout("ax") mode as u16 => _,
        );
    }
}

pub fn exit() {
    unsafe {
        asm!(
            "int 21h",
            inout("ax") 0x4C00 => _,
        );
    }
}
