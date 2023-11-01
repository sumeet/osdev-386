#!/bin/bash
set -ex

pushd smtos
cargo build -Zbuild-std --release --target=i386-unknown-none.json
cp target/i386-unknown-none/release/smtos ../smtos.bin
popd
xxd smtos.bin
ls -lah smtos.bin
qemu-system-i386 -cpu qemu32 -fda smtos.bin -display gtk,zoom-to-fit=on
