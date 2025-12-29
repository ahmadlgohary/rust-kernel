// We are building an independent OS thus we don't include
// the std library since it relies on OS services
#![no_std]
// Disable the Rust entry point since we removed the
// underlying runtime system and we will define our own entry point
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use rust_kernel::println;
use core::panic::PanicInfo;


/*
* This is our custom entry point.
* The linker looks for a function called `_start` by default
* This is why we added the no_mangle attribute
*/
#[unsafe(no_mangle)] // do not mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("hello again{}\n", "!");
    #[cfg(test)]
    test_main();
    
    #[allow(clippy::empty_loop)]
    loop {}
}

#[cfg(not(test))]   // our panic handler in normal builds
#[panic_handler]    // our own custom panic handler
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_kernel::test_panic_handler(info)
}