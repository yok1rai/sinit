#!/usr/bin/env bash
set -e

echo "==> Building sinit..."
cargo build --release --target x86_64-unknown-linux-musl

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
