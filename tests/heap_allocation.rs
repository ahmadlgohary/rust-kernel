#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use x86_64::VirtAddr;

use rust_kernel::allocator;
use rust_kernel::memory::{self, BootInfoFrameAllocator};
use core::panic::PanicInfo;

entry_point!(main);

fn main(boot_info: &'static BootInfo)->!{

    rust_kernel::init();
    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe {memory::init(physical_memory_offset)};
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    test_main();
    rust_kernel::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_kernel::test_panic_handler(info)
}

#[test_case]
fn simple_allocation() {
    use alloc::boxed::Box;
    let heap_value_1 = Box::new(41);
    let heap_value_2 = Box::new(67);
    assert_eq!(*heap_value_1, 41);
    assert_eq!(*heap_value_2, 67);
}


#[test_case]
fn large_vector_allocation() {
    use alloc::vec::Vec;
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n-1)*(n/2));
}

#[test_case]
fn many_boxes(){
    use rust_kernel::allocator::HEAP_SIZE;
    use alloc::boxed::Box;
    for i in 0..HEAP_SIZE{
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}

#[test_case]
fn many_boxes_long_lived(){
    /// ---------------------------- ///
    /// * fails for bump allocator * ///
    /// ---------------------------- ///
    use rust_kernel::allocator::HEAP_SIZE;
    use alloc::boxed::Box;
    let long_lived = Box::new(67);
    for i in 0..HEAP_SIZE{
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
    assert_eq!(*long_lived, 67);
}