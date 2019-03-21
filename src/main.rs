#![no_std]
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;

    write!(
        vga_buffer::WRITER.lock(),
        "The numbers are {} and {} and this is a really long line so what happens now",
        42,
        1.0 / 3.0
    )
    .unwrap();

    loop {}
}
