#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(RustOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use RustOS::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hello");
    RustOS::init();

    let ptr = 0xdeadbeef as *mut u32;
    unsafe{*ptr = 42;}

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    RustOS::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    RustOS::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    RustOS::test_panic_handler(info);
}