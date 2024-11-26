#!/bin/bash

# Run the kernel in QEMU
qemu-system-x86_64 \
    -drive format=raw,file=target/x86_64-unknown-none/release/LUX-OS \
    -serial stdio \
    -display none