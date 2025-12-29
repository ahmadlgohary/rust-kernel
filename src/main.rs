// We are building an independent OS thus we don't include
// the std library since it relies on OS services
#![no_std]

// Disable the Rust entry point since we removed the
// underlying runtime system and we will define our own entry point
#![no_main]

mod vga_buffer; // Import our vga printing module
use core::panic::PanicInfo;

// do not mangle the name of this function
#[unsafe(no_mangle)] 

/*
 * This is our custom entry point.
 * The linker looks for a function called `_start` by default
 * This is why we added the no_mangle attribute
 */
 pub extern "C" fn _start() -> ! {
    println!("hello again{}\n","!");
    panic!("Some Panic Message");
    loop {}
}

// our own custom panic handler
#[panic_handler] 
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {}
}
