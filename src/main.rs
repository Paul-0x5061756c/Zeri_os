#![no_std]
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;

use crate::vga_buffer::Color;
use crate::vga_buffer::ColorCode;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println_colored!(ColorCode::new(Color::Red, Color::Black), "{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello world\nare we writing now?{}", "!");
    panic!("WAAH");
    loop {}
}
