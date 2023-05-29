#![no_std] // avoid linking to std
#![no_main] // remove dependency to rust runtime

mod vga_buffer;

use core::panic::PanicInfo;

/// this function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// entry point of the operation system since the linker looks for a
/// function named `_start` by default
/// the `#[no_mangle]` attribute is necessary to avoid turning the name 
/// of this function to some cryptic type.
#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("Hello World");

    loop {}
}
