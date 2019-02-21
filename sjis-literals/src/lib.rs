extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse, LitStr};
use encoding_rs::SHIFT_JIS;

#[proc_macro]
pub fn sjis(input: TokenStream) -> TokenStream {
    let string_literal: LitStr = match parse(input) {
        Ok(literal) => literal,
        Err(_) => panic!("Something other than a string literal was provided.")
    };
    let string = string_literal.value();
    let (bytes, _, error) = SHIFT_JIS.encode(&string);
    if error {
        panic!("The string literal contains a character that is not representable.");
    }
    let mut output = String::from("[");
    for (i, &byte) in bytes.iter().enumerate() {
        output.push_str(&format!("0x{:X}", byte));
        if i == 0 {
            output.push_str("u8");
        }
        output.push(',');
    }
    output.push(']');
    output.parse().unwrap()
}