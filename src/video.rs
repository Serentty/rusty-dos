use rusty_asm::rusty_asm;

pub fn fill_screen(c: u8) {
    unsafe {
        rusty_asm! {
            let color: in("{dl}") = c;
            clobber("ax");
            asm("volatile", "intel") {
                "mov   ax, 0A000h
                 mov   es, ax
                 xor   di, di
                 mov   cx, 320*200/2
                 mov   al,$color
                 mov   ah, al
                 rep   stosw
                 
                 ret"
            }
        }
    }
}

pub fn plot_pixel(x: u16, y: u16, color: u8) {
    unsafe {
        rusty_asm! {
            let c: in("{al}") = color as u8;
            let mut px: in("{cx}") = x as u16;
            let mut py: in("{dx}") = y as u16;
            clobber("bh");
            asm("volatile", "intel") {
                "mov   ah,0ch
                 mov   bh,0
                 int   10h"
            }
        }
    }
}