use crate::dos;

pub fn write(bytes: &[u8]) {
    for &byte in bytes {
        dos::print_character(byte);
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::write($($arg)*));
}
