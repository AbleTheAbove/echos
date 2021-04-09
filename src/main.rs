#![no_std]
#![no_main]
#![feature(llvm_asm)] //TODO(Able): Convert to asm macro instead
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(echos::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::{env, panic::PanicInfo};
use echos::{draw, logln};
const OS_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(debug_assertions)]
const BUILD_PROFILE: &str = "debug";

#[cfg(not(debug_assertions))]
const BUILD_PROFILE: &str = "release";

// TODO(Able): Move to the klib
pub mod kernel_state;
pub use kernel_state::KERNELSTATE;
// ENDTODO
#[no_mangle]
pub extern "C" fn _start() -> ! {
    echos::init(); // NOTE(Able): Initialize the interrupt table
    KERNELSTATE.lock().terminal = 0;

    #[cfg(test)]
    test_main();
    /*
    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }
    // FIX(Able): Properly handle stack overflow
    // uncomment line below to trigger a stack overflow
    //    stack_overflow();
    */
    term0_draw();

    match KERNELSTATE.lock().terminal {
        0 => {}
        1 => draw(),
        _ => {}
    }

    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    logln!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    echos::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

fn term0_draw() {
    let seperator =
        "================================================================================";
    let banner = r#"
                                 888       .d88888b.   .d8888b.
                                 888      d88P" "Y88b d88P  Y88b
                                 888      888     888 Y88b.
                .d88b.   .d8888b 88888b.  888     888  "Y888b.
               d8P  Y8b d88P"    888 "88b 888     888     "Y88b.
               88888888 888      888  888 888     888       "888
               Y8b.     Y88b.    888  888 Y88b. .d88P Y88b  d88P
                "Y8888   "Y8888P 888  888  "Y88888P"   "Y8888P"

"#;
    logln!("{}{}", banner, seperator);
    logln!("Version: {} {}\n{}", OS_VERSION, BUILD_PROFILE, seperator);
}
