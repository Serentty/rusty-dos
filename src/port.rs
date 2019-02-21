#![allow(unused_assignments)] 

use rusty_asm::rusty_asm;

#[inline(always)]
pub unsafe fn inb(port: u16) -> u8 {
    rusty_asm! {
        let port: in("{dx}") = port;
        let mut value: out("{al}") = 0_u8;
        asm("volatile", "intel") {
            "in $value, dx"
        }
        value
    }
}

#[inline(always)]
pub unsafe fn inw(port: u16) -> u16 {
    rusty_asm! {
        let port: in("{dx}") = port;
        let mut value: out("{ax}") = 0_u16;
        asm("volatile", "intel") {
            "in $value, dx"
        }
        value
    }
}

#[inline(always)]
pub unsafe fn inl(port: u16) -> u32 {
    rusty_asm! {
        let port: in("{dx}") = port;
        let mut value: out("{eax}") = 0_u32;
        asm("volatile", "intel") {
            "in $value, dx"
        }
        value
    }
}

#[inline(always)]
pub unsafe fn outb(port: u16, value: u8) {
    rusty_asm! {
        let port: in("{dx}") = port;
        let value: in("{al}") = value;
        asm("volatile", "intel") {
            "out dx, al"
        }
    }
}

#[inline(always)]
pub unsafe fn outw(port: u16, value: u16) {
    rusty_asm! {
        let port: in("{dx}") = port;
        let value: in("{ax}") = value;
        asm("volatile", "intel") {
            "out dx, ax"
        }
    }
}

#[inline(always)]
pub unsafe fn outl(port: u16, value: u32) {
    rusty_asm! {
        let port: in("{dx}") = port;
        let value: in("{eax}") = value;
        asm("volatile", "intel") {
            "out dx, eax"
        }
    }
}

