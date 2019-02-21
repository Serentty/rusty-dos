use proc_macro::TokenStream;
use syn::{parse, LitStr};
use encoding_rs::SHIFT_JIS;

#[proc_macro]
pub fn sjis(input: TokenStream) -> TokenStream {
    let string: LitStr = parse(input).unwrap();
    let bytes = SHIFT_JIS.encode(string);
    let output = String::new("[");
    for (byte, i) in bytes.iter().enumerate() {
        output.push_str(&format!("0x{:X}", byte));
        if i == 0 {
            output.push_str("u8");
        }
        output.push(',');
    }
    output.parse().unwrap()
}