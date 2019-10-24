use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    fs::copy("com.ld", Path::new(&out_dir).join("com.ld"));
    fs::copy("startup.o", Path::new(&out_dir).join("startup.o"));
}
