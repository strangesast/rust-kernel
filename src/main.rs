#![no_std]
#![no_main]
mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &str = "Helloooooo";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("???! {} {}", HELLO, 42);

    loop {}
}
