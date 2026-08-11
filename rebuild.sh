#!/usr/bin/env bash
set -e

echo "==> Building sinit..."
cargo build --release --target x86_64-unknown-linux-musl --features unsafe

echo "==> Copying sinit into initramfs..."
cp target/x86_64-unknown-linux-musl/release/sinit initramfs/init
chmod +x initramfs/init

echo "==> Rebuilding initramfs..."
rm -f initramfs.cpio

(
    cd initramfs
    find . -print0 | cpio --null -ov --format=newc > ../initramfs.cpio
)

echo "==> Done!"

if [[ "$1" == "--run" || "$1" == "-r" ]]; then
    echo "==> Booting in QEMU..."
    qemu-system-x86_64 \
            -kernel /boot/vmlinuz-linux \
            -initrd initramfs.cpio \
            -append "console=ttyS0 rdinit=/init" \
            -m 1G \
            -nographic
fi
