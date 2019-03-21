#![no_std]
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!(
        "The numbers are {} and {} and this is a really long line so what happens now",
        42,
        1.0 / 3.0
    );

    panic!("OH SHIT");

    loop {}
}
