use crate::println;
#[repr(u32)]
pub enum RequestType {
    BasicInformation = 0x00000000,
    VersionInformation = 0x00000001,
    ThermalPowerManagementInformation = 0x00000006,
    StructuredExtendedInformation = 0x00000007,
    ExtendedFunctionInformation = 0x80000000,
    ExtendedProcessorSignature = 0x80000001,
    BrandString1 = 0x80000002,
    BrandString2 = 0x80000003,
    BrandString3 = 0x80000004,
    // reserved = 0x80000005,
    CacheLine = 0x80000006,
    TimeStampCounter = 0x80000007,
    PhysicalAddressSize = 0x80000008,
}

pub fn _cpuid(code: RequestType) -> (u32, u32, u32, u32) {
    let res1;
    let res2;
    let res3;
    let res4;

    unsafe {
        llvm_asm!("cpuid"
        : // output operands
        "={eax}"(res1),
        "={ebx}"(res2),
        "={ecx}"(res3),
        "={edx}"(res4)
        : // input operands
        "{eax}"(code as u32),
        "{ecx}"(0 as u32)
        : // clobbers
        : // options
        );
    }

    (res1, res2, res3, res4)
}

// NOTE(Able): Named uchar because it takes an u32 and converts it to a character
fn _uchar(a: u32) -> char {
    match core::char::from_u32(a) {
        Some(x) => x,
        None => panic!(),
    }
}

fn _cpuid_0() {
    let (res1, res2, res3, res4) = _cpuid(RequestType::BasicInformation);
    println!(
        "Regester 1: {}\nRegister 2: {}\nRegister 3: {}\nRegister 4: {}",
        res1, res2, res3, res4
    );
}
