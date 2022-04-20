#![allow(unused_assignments)] 
use core::arch::asm;

#[inline(always)]
pub unsafe fn inb(port: u16) -> u8 {
    let value;
    asm!(
        "in al, dx",
        in("dx") port,
        out("al") value,
    );
    value
}

#[inline(always)]
pub unsafe fn inw(port: u16) -> u16 {
    let value;
    asm!(
        "in ax, dx",
        in("dx") port,
        out("ax") value,
    );
    value
}

#[inline(always)]
pub unsafe fn inl(port: u16) -> u32 {
    let value;
    asm!(
        "in eax, dx",
        in("dx") port,
        out("eax") value,
    );
    value
}

#[inline(always)]
pub unsafe fn outb(port: u16, value: u8) {
    asm!(
        "out dx, al",
        in("al") value,
        in("dx") port,
    );
}

#[inline(always)]
pub unsafe fn outw(port: u16, value: u16) {
    asm!(
        "out dx, ax",
        in("ax") value,
        in("dx") port,
    );
}

#[inline(always)]
pub unsafe fn outl(port: u16, value: u32) {
    asm!(
        "out dx, eax",
        in("eax") value,
        in("dx") port,
    );
}

