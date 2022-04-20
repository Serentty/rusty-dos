use bitflags::bitflags;
use crate::port::{inb, outb};

bitflags! {
    pub struct Status: u8 {
        const FLAGA = 0b0000_0001;
        const FLAGB = 0b0000_0010;
        const BUSY  = 0b1000_0000;
    }
}

#[inline(always)]
pub fn write_address(address: u8) {
    unsafe { outb(0x188, address); }
}

#[inline(always)]
pub fn write_data(data: u8) {
    unsafe { outb(0x18A, data); }
}

#[inline(always)]
pub fn read_status() -> Status {
    Status::from_bits_truncate(unsafe { inb(0x188) })
}

#[inline(always)]
pub fn read_data() -> u8 {
    unsafe { inb(0x18A) }
}

#[inline(always)]
pub fn write(address: u8, data: u8) {
    write_address(address);
    write_data(data);
}
