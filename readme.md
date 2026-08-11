# Sinit

A lightweight, minimal init system (PID 1) written in Rust, designed for custom Linux environments and `initramfs`.

Developed by **yok1rai**.

---

## Project Structure

```text
src/
├── lib.rs        # Library interface exports
├── main.rs       # Entry point and PID 1 execution loop
├── mount.rs      # Virtual filesystem mounting logic
├── process.rs    # Process spawning, job control, and zombie reaping
├── signal.rs     # Signal masking and handling initialization
└── utils.rs      # High-resolution boot timing helpers
```

---

## Prerequisites

* **Rust toolchain** (stable)
* **MUSL Target:** Install via `rustup target add x86_64-unknown-linux-musl`
* **Build Tools:** `cpio`, `musl-tools`
* **Testing:** `qemu-system-x86_64` and a Linux kernel image (`vmlinuz`)

---

## Building

Run the included `./rebuild.sh` script to compile sinit against the MUSL target, stage it inside `initramfs/init`, and generate the initramfs.cpio archive:

```bash
chmod +x rebuild.sh
./rebuild.sh
```

### Testing with QEMU

Boot your image with a local Linux kernel:

```Bash
qemu-system-x86_64 \
    -kernel /path/to/vmlinuz \
    -initrd initramfs.cpio \
    -append "console=ttyS0 quiet" \
    -nographic
```
***(To exit QEMU in -nographic mode, press Ctrl+A then X).***

---

## License

Licensed under [GPL v3.0](https://www.google.com/search?q=./LICENSE).
