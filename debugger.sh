#!/bin/bash
set -ex
gdb -q -ex "target remote localhost:1234" -ex "layout asm" -ex "break *0x7C00" -ex "set disassembly-flavor intel" -ex "continue"
