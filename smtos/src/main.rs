#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let string = "Welcome to smt OS...\0";

    unsafe {
        asm!(
        "mov si, {0:x}",
        "3:",
        "lodsb",
        "test al, al",
        "jz 4f",
        "mov ah, 0x0E",
        "int 0x10",
        "jmp 3b",
        "4:",
        "jmp 4b",
        in(reg) string.as_ptr() as u16
        );
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
