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
use rust_kernel::task::keyboard;
// use rust_kernel::task::{Task, simple_executor::SimpleExecutor};
use rust_kernel::task::Task;
use rust_kernel::task::executor::Executor;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use x86_64::{structures::paging::Translate, VirtAddr};

extern crate alloc;
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};


entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World :) \n");
    
    rust_kernel::init();


    // ------------------------------------------------------------------
    // initializing Paging 
    // ------------------------------------------------------------------
    
    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe {rust_kernel::memory::init(physical_memory_offset)};
    let mut frame_allocator = unsafe {
        rust_kernel::memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    
    rust_kernel::allocator::init_heap(&mut mapper, &mut frame_allocator)
    .expect("heap initialization failed");

    #[cfg(test)]
    test_main();
    
    println!("\nIt did not Crash");
    
    println!("\nAsynchronous Executor is now running!");
    
    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();    

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

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async num {number}")
}