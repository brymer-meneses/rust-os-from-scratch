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
    static HELLO: &[u8] = b"Hello World!";
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            // write the string byte per byte
            *vga_buffer.offset(i as isize * 2) = byte;
            // set color byte to `0xb` which colors it as cyan
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
