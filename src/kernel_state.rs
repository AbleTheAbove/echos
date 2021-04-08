use lazy_static::lazy_static;
use spin::Mutex;

pub struct KernelState {
    pub terminal: u8,
}

lazy_static! {
    pub static ref KERNELSTATE: Mutex<KernelState> = Mutex::new(KernelState { terminal: 1 });
}
