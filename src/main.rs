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
use bootloader::{BootInfo, entry_point};
use x86_64::VirtAddr;


/*
* This is our custom entry point.
* The linker looks for a function called `_start` by default
* This is why we added the no_mangle attribute
*/
entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("hello again{}\n", "!");
    
    rust_kernel::init();

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut _mapper = unsafe {rust_kernel::memory::init(physical_memory_offset)};
    let mut _frame_allocator = unsafe {
        rust_kernel::memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };



    #[cfg(test)]
    test_main();
    
    println!("It did not Crash");

    rust_kernel::hlt_loop();
}

#[cfg(not(test))]   // our panic handler in normal builds
#[panic_handler]    // our own custom panic handler
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    rust_kernel::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_kernel::test_panic_handler(info)
}