#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    for i in 0..10 {
        println("yoooo we can call function 4");
    }

    loop {}
}

fn println(s: &str) {
    print_str(s);
    print_str("\r\n");
}

fn print_str(s: &str) {
    print_bytes(s.as_bytes());
}

fn print_bytes(bs: &[u8]) {
    for i in 0..bs.len() {
        unsafe {
            asm!(
                "int 0x10",
                in("al") bs[i],
                in("ah") 0x0Eu8,
            );
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

//////////////////////////////////////////////////////////////
// Things that have been suggested that will probably be needed later
//////////////////////////////////////////////////////////////

// set up the stack (TODO: not doing this yet, see end of file)
// unsafe {
// // set the stack pointer to the top of addressable memory (1MB of memory)
// // ALSO: chatgpt thinks setting this to 0xFFFF0 might be better for performance 'cause
// //       of alignment, but idk
//
// // TODO: can't get this to work yet, but
// // also function calls seem to work fine without this?
// //
// // actually they don't seem to work WITH this,
// // which seems odd. we'll leave this here for now,
// // cause the whole thing seems suspicious
//
// // shelving this for now, because comp works so far
//
// // asm!(
// //     "mov ax, 0xff",
// //     "mov ss, ax",
// // );
// // asm!("mov sp, 0xFFF0");
// }

//////////////////////////////////////////////////////////////
// writing text directly to vga_buffer, should be faster than
// interrupting char by char apparently:
/////////////////////////////////////////////////////////////
// const VGA_BUFFER: u32 = 0xb8000;
//
// let vga = VGA_BUFFER as *mut u16;
//
// unsafe {
// *vga.offset(0) = ('A' as u16) | (0x21 << 8);
// }
// unsafe {
// // let vga = 0xb8000 as *mut u16;
// // ptr::write_volatile(vga, 'A' as u16 + (15 << 8));
// // *vga = 'A' as u16 + (15 << 8); // ASCII 'A' in white on black
// // *vga.offset(1) = 'B' as u16 + (15 << 8); // ASCII 'B' in white on black
// // *vga.offset(2) = 'C' as u16 + (15 << 8); // ASCII 'C' in white on black
// }
