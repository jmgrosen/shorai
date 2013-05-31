SRCDIR := src
ODIR := obj
ISODIR := isodir
TOOLSDIR := tools

RSFILES := $(shell find $(SRCDIR) -type f -name "*.rs")
RSKERNEL := $(SRCDIR)/kernel.rc
BOOTASM := $(SRCDIR)/boot.asm
OBJFILES := $(ODIR)/boot.o $(ODIR)/kernel.o

# Copyright (c) 2013 John Grosen
# Licensed under the terms described in the LICENSE file in the root of this repository

TOOLKIT_PREFIX := i586-elf
LD := $(TOOLKIT_PREFIX)-ld
RUSTC := rustc
AS := nasm
RSFLAGS := -O --target i386-intel-linux --lib -L $(SRCDIR)
LFLAGS := -melf_i386
ASFLAGS := -f elf32

.PHONY: all run clean

all: shorai.iso

run: shorai.iso
	qemu-system-i386 -cdrom shorai.iso

shorai.iso: $(ODIR)/shorai.bin
	mkdir -p $(ISODIR)
	cp -R $(TOOLSDIR)/boot $(ISODIR)
	cp $(ODIR)/shorai.bin $(ISODIR)/boot/shorai.bin
	mkisofs -quiet -R -b boot/grub/stage2_eltorito -no-emul-boot \
	        -boot-load-size 4 --boot-info-table -o shorai.iso $(ISODIR)

$(ODIR)/shorai.bin: $(OBJFILES) src/linker.ld
	$(LD) $(LFLAGS) -T src/linker.ld -o $(ODIR)/shorai.bin $(OBJFILES)

$(ODIR)/kernel.o: $(SRCDIR)/kernel.rc $(RSFILES)
	@mkdir -p $(@D)
	$(RUSTC) $(RSFLAGS) -c $< -o $@

$(ODIR)/%.o: $(SRCDIR)/%.asm
	@mkdir -p $(@D)
	$(AS) $(ASFLAGS) -o $@ $<

clean:
	-$(RM) $(wildcard $(OBJFILES) shorai.iso $(ODIR)/shorai.bin)
	-$(RM) -r $(ODIR) $(ISODIR)
