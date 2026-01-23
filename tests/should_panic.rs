#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_kernel::{QemuExitCode, exit_qemu, serial_print, serial_println, hlt_loop};

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    hlt_loop();
    // ToDo
    // can implement unwinding using this library 
    // so we can test more than 1 should_panic function
    // https://github.com/nbdd0121/unwinding/tree/trunk
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests{
        test();
        serial_println!("[test did not panic]");
        exit_qemu(QemuExitCode::Failed);
    }
    exit_qemu(QemuExitCode::Success);
    
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    hlt_loop();
}

#[test_case]
fn should_panic(){
    serial_print!("should_panic::should fail...\t");
    assert_eq!(1,0);
}