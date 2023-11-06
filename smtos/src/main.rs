#![feature(asm_const)]
#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    for i in 0..20 {
        // print(" yoooo ");
        print_num(i);
        // println("");
    }
    loop {}
}

#[inline(always)]
unsafe fn set_up_stack() {
    asm!(
        "cli",
        "mov ax, cs",
        "mov ds, ax",
        "mov es, ax",
        "mov ax, 0",
        "mov ss, ax",
        "mov sp, 0xFFFF",
        "sti",
    )

    // asm!(
    //     // Set up the segment registers with the proper data selector
    //     "mov ax, {data_segment}",
    //     "mov ds, ax",
    //     "mov es, ax",
    //     "mov fs, ax",
    //     "mov gs, ax",
    //
    //     // Set up the stack segment
    //     "mov ss, ax",
    //
    //     // Set the stack pointer to the top of the stack
    //     // Assuming a stack at 0x90000, the stack grows downwards
    //     "mov sp, {stack_top}",
    //
    //     // You may need to align the stack pointer here if required
    //     // For example, to align to a 16-byte boundary:
    //     // "and sp, 0xFFF0",
    //
    //     data_segment = const 0x9000, // Data segment selector
    //     stack_top = const 0xFFFF,    // Stack top (stack grows downwards)
    //     options(nostack)
    // );
}

fn print_num(mut n: u16) {
    let mut buf = [0u8; 10];
    let mut i = 0;
    while n > 0 {
        buf[i] = (n % 10) as u8;
        n /= 10;
        i += 1;
    }
    // handle n == 0
    if i == 0 {
        i = 1;
    }
    for j in (0..i).rev() {
        write_char(buf[j] + b'0');
    }
}

fn println(s: &str) {
    print(s);
    print("\r\n");
}

fn print(s: &str) {
    print_bytes(s.as_bytes());
}

fn print_bytes(bs: &[u8]) {
    for i in 0..bs.len() {
        write_char(bs[i]);
    }
}

fn write_char(c: u8) {
    unsafe {
        asm!(
            "int 0x10",
            in("al") c,
            in("ah") 0x0Eu8,
        );
    }
}

// INT 16,0 (INT 16h, AH=00h): This function waits for a keypress and returns the ASCII code in the
// AL register and the scan code in the AH register. It works with the most basic set of keyboard
// functionalities and is compatible with all PCs.
//
// INT 16,10h (INT 16h, AH=10h): This enhanced keyboard function also waits for a keypress and
// returns the ASCII code in the AL register and the scan code in the AH register, but it is
// designed to work with the extended keys found on AT and PS/2 keyboards, such as function keys
// and other special keys not present on the original PC keyboard.
//
// The "extended" in this context refers to the extended keyboard (101/102 keys) support, which
// includes additional keys like F11, F12, Print Screen, Ctrl+Break, and others that were not part
// of the original IBM PC keyboard (which had 83/84 keys). This interrupt function was necessary to
// properly handle these new inputs that were not part of the earlier hardware.
//
// for us, for now, sticking with the basic one so that we are more compatible with older hardware
fn block_read_kb() -> u8 {
    let mut c: u8;
    unsafe {
        asm!(
            "int 0x16",
            out("al") c,
            in("ah") 0x00u8,
        );
    }
    c
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
