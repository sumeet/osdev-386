/* linker.ld */
OUTPUT_FORMAT(binary)
ENTRY(_start)

SECTIONS
{
    . = 0x7C00;

    .boot : AT(0x7C00) {
        *(.boot)
    }

    .text : {
        *(.text)
    }

    .rodata : {
        *(.rodata)
    }

    .data : {
        *(.data)
    }

    .bss : {
        *(.bss)
    }

    . = 0x7DFE; /* Sets the location counter to 0x1FE bytes into the 0x7C00 offset */
    .sig : AT(0x7DFE) {
        SHORT(0xAA55); /* Magic number */
    }
}
