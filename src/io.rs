use crate::dos;

pub fn write(bytes: &[u8]) {
    for &byte in bytes {
        dos::print_character(byte);
    }
}