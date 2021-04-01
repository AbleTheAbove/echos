#![no_std]
#![no_main]
#![feature(llvm_asm)] //TODO(Able): Convert to asm macro instead
#![feature(custom_test_frameworks)]
#![test_runner(aura_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::env;
use core::panic::PanicInfo;

use aura_os::println;

mod arch;

// TODO(any): Make this architecture dependent
use arch::x86_64::cpuid::{RequestType, _cpuid};

const OS_VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[cfg(debug_assertions)]
const BUILD_PROFILE: &'static str = "debug";

#[cfg(not(debug_assertions))]
const BUILD_PROFILE: &'static str = "release";

struct KernelState {}

#[no_mangle]
pub extern "C" fn _start() -> ! {
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
    println!("{}{}", banner, seperator);
    println!("Version: {} {}\n{}", OS_VERSION, BUILD_PROFILE, seperator);

    let kernel_state = KernelState {};

    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
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
