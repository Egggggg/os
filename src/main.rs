#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(beeos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use beeos::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hiii :3");

    beeos::init();

    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    }

    #[cfg(test)]
    test_main();

    println!("no crashe");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use beeos::test_panic_handler;

    test_panic_handler(info);
    loop {}
}
