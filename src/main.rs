#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(RustOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;

use core::panic::PanicInfo;
use RustOS::println;
use bootloader::{BootInfo, entry_point};
use x86_64::{structures::paging::PageTable, VirtAddr};

entry_point!(kernel_main);

fn kernel_main(boot_info : &'static BootInfo) -> ! {
    use RustOS::memory::{self, BootInfoFrameAllocator};
    use RustOS::allocator;


    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset)};
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    let x = Box::new(41);

    println!("hello");
    RustOS::init();
    
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