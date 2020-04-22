#![feature(proc_macro_hygiene)]
#![feature(asm)]
#![no_main]
#![no_std]

mod dos;
mod panic;
mod io;
mod port;
mod opn;
mod util;
mod video;

#[no_mangle]
pub unsafe extern "C" fn start() {
    util::seed_random();
    dos::set_video_mode(0x13);

    for i in 0..10000 {
        let x: u16 = util::random() as u16 % 320;
        let y: u16 = util::random() as u16 % 200;
        let color: u8 = util::random() as u8 % 255;

        video::plot_pixel(x, y, color);
    }

    loop {
        let code = dos::get_keyboard_input();
        if code != 0 { break; }
    }

    dos::set_video_mode(0x03);
    print!(b"Thanks for trying Rusty DOS!");
}
