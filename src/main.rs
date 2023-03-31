#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(beeos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use beeos::{println, hlt_loop};
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use beeos::memory::{BootInfoFrameAllocator, self};
    use x86_64::{VirtAddr};

    println!("hiii :3");

    beeos::init();
    
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    #[cfg(test)]
    test_main();

    println!("no crashe");
    hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use beeos::test_panic_handler;

    test_panic_handler(info);
    hlt_loop();
}
