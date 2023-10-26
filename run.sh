#!/bin/bash
set -ex

nasm -f bin -o hello.bin hello.asm
dd if=hello.bin of=hello.img bs=512 count=1
qemu-system-i386 -cpu qemu32 -fda hello.img
