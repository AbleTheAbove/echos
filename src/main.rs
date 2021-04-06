#![no_std]
#![no_main]
#![feature(llvm_asm)] //TODO(Able): Convert to asm macro instead
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(aura_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
use aura_os::{draw, logln};
use core::env;
use core::panic::PanicInfo;
const OS_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(debug_assertions)]
const BUILD_PROFILE: &str = "debug";

#[cfg(not(debug_assertions))]
const BUILD_PROFILE: &str = "release";

pub struct KernelState {
    pub terminal: u8,
}

// TODO(Able): Move to the klib
use lazy_static::lazy_static;
use spin::Mutex;
lazy_static! {
    pub static ref KERNELSTATE: Mutex<KernelState> = Mutex::new(KernelState { terminal: 1 });
}
// ENDTODO
#[no_mangle]
pub extern "C" fn _start() -> ! {
    aura_os::init(); // NOTE(Able): Initialize the interrupt table

    KERNELSTATE.lock().terminal = 1;
    #[cfg(test)]
    test_main();

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

/// Tdraws function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    logln!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    aura_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

fn term0_draw() {
    let seperator =
        "================================================================================";
    let banner = r#"
             .d888888                              .88888.  .d88888b
             d8'    88                             d8'   `8b 88.    "'
             88aaaaa88a dP    dP 88d888b. .d8888b. 88     88 `Y88888b.
             88     88  88    88 88'  `88 88'  `88 88     88       `8b
             88     88  88.  .88 88       88.  .88 Y8.   .8P d8'   .8P
             88     88  `88888P' dP       `88888P8  `8888P'   Y88888P
"#;
    logln!("{}{}", banner, seperator);
    logln!("Version: {} {}\n{}", OS_VERSION, BUILD_PROFILE, seperator);
}
