use crate::dos;

pub fn write_bytes(bytes: &[u8]) {
    bytes.iter().for_each(|&b| dos::print_character(b));
}

pub fn write_str(s: &str) {
    s.chars().for_each(|c| dos::print_character(crate::text::cp437::encode_char_lossy(c)));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::write_str($($arg)*));
}
