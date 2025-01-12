#!/bin/bash

qemu-system-x86_64 \
    -drive file=disk.img,format=raw \
    -display gtk \
    -monitor stdio

    # Wait a moment for QEMU to start
sleep 1

# Connect with GDB
gdb -q -ex "target remote localhost:1234" \
    -ex "set disassembly-flavor intel" \
    -ex "break *0x7c00" \
    -ex "continue" \
    -ex "x/10i 0x7c00"
