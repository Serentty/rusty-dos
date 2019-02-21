use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out_dir.join("com.ld")).unwrap().write_all(include_bytes!("com.ld")).unwrap();
    File::create(out_dir.join("startup.o")).unwrap().write_all(include_bytes!("startup.o")).unwrap();
}
