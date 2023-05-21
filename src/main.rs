
#![no_std] // avoid linking to std
#![no_main] // remove dependency to rust runtime

use core::panic::PanicInfo;

/// this function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// entry point of the operation system since the linker looks for a
/// function named `_start` by default
/// the `#[no_mangle]` attribute is necessary to avoid turning the name 
/// of this function to some cryptic type.
#[no_mangle]
pub extern "C" fn _start() -> ! {

    loop {}
}
