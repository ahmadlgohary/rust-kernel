// We are building an independent OS thus we don't include 
// the std library since it relies on OS services
#![no_std]

// Disable the Rust entry point since we removed the 
// underlying runtime system and we will define our own entry point
#![no_main]

use core::panic::PanicInfo;


#[unsafe(no_mangle)] // do not mangle the name of this function
pub extern "C" fn _start() -> ! {
    /*
    This is our custom entry point.
    The linker looks for a function called `_start` by default
    This is why we added the no_mangle attribute
    */
    loop {}
}


#[panic_handler] // our own custom panic handler
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}