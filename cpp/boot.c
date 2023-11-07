__asm__(".code16gcc");
__asm__("jmp _start");

void write_char(unsigned char c) {
  __asm__("int 0x10" : : "a"((0x0e << 8) | c));
}


void _start() {
  for (int i = 0; i < 10000; i++) {
    write_char('H');
  }
  while(1) {}
}


// TODO: for now, works without setting up the stack, but we might need this later
//    __asm__(
//      ".intel_syntax noprefix\n"  // Switch to Intel syntax without prefixes
//        "cli\n"                     // Clear interrupts
//        "mov ax, 0x9000\n"          // Stack segment
//        "mov ss, ax\n"
//        "mov sp, 0xFFFF\n"          // Stack pointer
//        "sti\n"                     // Set interrupts
//        ".att_syntax prefix\n"      // Switch back to AT&T syntax
//    );
