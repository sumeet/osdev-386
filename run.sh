#!/bin/bash
set -ex

pushd smtos
cargo build -Zbuild-std --release --target=i386-unknown-none.json
cp target/i386-unknown-none/release/smtos ../smtos.bin
popd
xxd smtos.bin
ls -lah smtos.bin
GDB_FLAGS=""
if [ "$DEBUG" ]; then
    GDB_FLAGS="-s -S"
fi
#qemu-system-i386 -cpu qemu32 -fda smtos.bin $GDB_FLAGS -display gtk,zoom-to-fit=on
qemu-system-i386 -cpu qemu32 -fda smtos.bin $GDB_FLAGS -nographic
objdump -M intel -D -b binary -m i386 smtos.bin
