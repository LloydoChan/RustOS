#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(RustOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use RustOS::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello {}", 42);
    
    #[cfg(test)]
    test_main();

    loop{}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    RustOS::test_panic_handler(info);
}