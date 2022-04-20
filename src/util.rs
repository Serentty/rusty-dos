use core::arch::asm;

pub fn random() -> u16 {
    let value;
    unsafe {
        asm!(
            "mov bx, [11h]",
            "mov ax, bx",
            "shl bx, 7",
            "xor ax, bx",
            "mov bx, ax",
            "shr bx, 9",
            "xor ax, bx",
            "mov bx, ax",
            "shl bx, 8",
            "xor ax, bx",
            "mov [11h], ax",
            out("ax") value,
            out("bx") _,
            out("cx") _,
            out("dx") _,
        );
        value
    }
}

pub fn seed_random() {
    unsafe {
        asm!(
            "int 1ah",
            "mov [11h], dx",
            "xor dx, dx",
            inout("ax") 0 => _,
            inout("cx") 0 => _,
            out("dx") _,
        );
    }
}
