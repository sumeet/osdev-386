[org 0x7c00] ; MBR start address

mov si, hello_string ; Point SI to the start of the string

print_loop:
    lodsb                  ; Load byte at DS:SI into AL and increment SI
    test al, al            ; Test if AL is zero (end of string)
    jz done                ; Jump to 'done' if zero
    mov ah, 0x0E           ; BIOS command to print character in AL
    int 0x10               ; BIOS interrupt
    jmp print_loop         ; Repeat for next character

done:
    jmp $ ; infinite loop


hello_string db 'Welcome to smt OS...', 0
times 510-($-$$) db 0  ; Pad with zeros up to 510 bytes
dw 0xAA55              ; Boot signature

