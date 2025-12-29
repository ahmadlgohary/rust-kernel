// We are building an independent OS thus we don't include
// the std library since it relies on OS services
#![no_std]
// Disable the Rust entry point since we removed the
// underlying runtime system and we will define our own entry point
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]


mod vga_buffer; // Import our vga printing module
mod serial;
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

#[cfg(test)]        // our panic handler in test mode
#[panic_handler]    // our own custom panic handler
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[Failed]\n");
    serial_println!("Error {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion(){
    assert_eq!(1, 1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port= Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where 
T:Fn()
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}