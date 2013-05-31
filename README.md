Shorai
======

Shorai is an amazing OS that will bring world peace,
cure cancer, and have <20 ms audio latency.

Building
--------

Requirements:

 * the master branch of [rust](https://github.com/mozilla/rust)
 * GNU binutils cross-compiled for i586-elf (the instructions at
   [the OSDev Wiki](http://wiki.osdev.org/GCC_Cross-Compiler)
   can be very helpful)
 * `nasm`
 * `mkisofs`, from `cdrtools`
 * a copy of `stage2_eltorito` from GRUB legacy in the `tools/boot/grub/` directory

For running it, you'll also need an x86 emulator (I'm using QEMU)

To build it, `make` should be enough. To run it, simply `make run`.
