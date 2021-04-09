#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(echos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use echos::logln;
use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    echos::test_panic_handler(info)
}

#[test_case]
fn test_logln() {
    logln!("test_logln output");
}
