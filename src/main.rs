// We are building an independent OS thus we don't include 
// the std library since it relies on OS services
#![no_std]

// Disable the Rust entry point since we removed the 
// underlying runtime system and we will define our own entry point
#![no_main]

// Import our vga printing module
mod vga_buffer;

use core::panic::PanicInfo;


#[unsafe(no_mangle)] // do not mangle the name of this function
pub extern "C" fn _start() -> ! {
    /*
    * This is our custom entry point.
    * The linker looks for a function called `_start` by default
    * This is why we added the no_mangle attribute
    */

    vga_buffer::print_something();

    // basic loop to display text on the screen
    // let vga_buffer = 0xb8000 as *mut u8;
    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }

    loop {}
}


#[panic_handler] // our own custom panic handler
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}