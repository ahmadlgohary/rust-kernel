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

extern crate alloc;
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};


entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("hello again{}\n", "!");
    
    rust_kernel::init();


    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe {rust_kernel::memory::init(physical_memory_offset)};
    let mut frame_allocator = unsafe {
        rust_kernel::memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    rust_kernel::allocator::init_heap(&mut mapper, &mut frame_allocator)
    .expect("heap initialization failed");

    let heap_value = Box::new(67);
    println!("heap_value at {heap_value:p}");
    
    let mut vec = Vec::new();
    for i in 0..500{
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    let reference_counted = Rc::new(vec![1,2,3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));



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