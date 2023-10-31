/* linker.ld */
OUTPUT_FORMAT(binary)
ENTRY(_start)

SECTIONS
{
    . = 0x7c00;
    .boot :
    {
        KEEP(*(.text))
        KEEP(*(.rodata))
        KEEP(*(.data))
        KEEP(*(.bss))
        . = 0x1FE;
        SHORT(0xAA55)
    }
}
