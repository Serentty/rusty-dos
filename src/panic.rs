use core::panic::PanicInfo;

use crate::dos;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    dos::print("\nPanic!$".as_ptr());
    dos::exit();
    loop {}
}
