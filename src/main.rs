#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(beeos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use beeos::{println, hlt_loop};
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hiii :3");

    beeos::init();

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
