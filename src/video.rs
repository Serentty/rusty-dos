pub fn fill_screen(color: u8) {
    unsafe {
        asm!(
            "mov   es, ax",
            "xor   di, di",
            "mov   cx, 320*200/2",
            "mov   al, dl",
            "mov   ah, al",
            "rep   stosw",
            inout("ax") 0xA000 => _,
            in("dl") color,
        )
    }
}

pub fn plot_pixel(x: u16, y: u16, color: u8) {
    unsafe {
        asm!(
            "int 10h",
            in("ax") (0x0C00u16) | (color as u16),
            in("bh") 0u8,
            in("cx") x,
            in("dx") y,
        );
    }
}