#![no_std]
#![feature(default_alloc_error_handler)]

#[no_mangle]
pub unsafe extern "C" fn handle() {
    panic!("I just panic every time")
}

#[no_mangle]
pub unsafe extern "C" fn init() {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    core::arch::wasm32::unreachable();
}
