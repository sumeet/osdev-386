#!/bin/bash
set -ex
rm -f boot.bin boot.o

i686-elf-gcc -masm=intel -nostdlib -nostartfiles -Wall -Wextra -pedantic -std=gnu99 -ffreestanding -fno-asynchronous-unwind-tables -fomit-frame-pointer -fno-pic -Os -m16 -xc -mpreferred-stack-boundary=2 -o boot.o boot.c

i686-elf-gcc -m16 -nostdlib -Wl,--build-id=none -Wl,-Tlinker.ld -o boot.bin boot.o

GDB_FLAGS=""
if [ "$DEBUG" ]; then
    GDB_FLAGS="-s -S"
fi

if [ -z "$BUILD_ONLY" ]; then
#qemu-system-i386 -cpu qemu32 -fda boot.bin $GDB_FLAGS -display gtk,zoom-to-fit=on
  qemu-system-i386 -cpu qemu32 -fda boot.bin $GDB_FLAGS -nographic ; reset
fi
objdump -M intel -D -b binary -m i386 boot.bin
