#![no_std]
#![no_main]
#![test_runner(RustOS::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]

use core::panic::PanicInfo;
use RustOS::{println, serial_print, serial_println};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    RustOS::test_panic_handler(info);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();
    loop{}
}

#[test_case]
fn test_println() {
    serial_print!("test_println... ");
    println!("test_println output");
    serial_println!("[ok]");
}