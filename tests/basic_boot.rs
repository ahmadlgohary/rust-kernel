#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_kernel::println;

/*
* This is our custom entry point.
* The linker looks for a function called `_start` by default
* This is why we added the no_mangle attribute
*/
#[unsafe(no_mangle)] // do not mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    #[allow(clippy::empty_loop)]
    loop {}
}


#[panic_handler] // our own custom panic handler
fn panic(info: &PanicInfo) -> ! {
    rust_kernel::test_panic_handler(info);
    loop {}
}

#[test_case]
fn test_println(){
    println!("test_println output");
}