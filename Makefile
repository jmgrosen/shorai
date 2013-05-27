SRCDIR := src
IDIR := include
ODIR := obj
ISODIR := isodir
TOOLSDIR := tools

CFILES := $(shell find $(SRCDIR) -type f -name "*.c")
SFILES := $(shell find $(SRCDIR) -type f -name "*.asm")
OBJFILES := $(patsubst $(SRCDIR)/%.asm,$(ODIR)/%.o,$(SFILES)) \
	    $(patsubst $(SRCDIR)/%.c,$(ODIR)/%.o,$(CFILES))

TOOLKIT_PREFIX := i586-elf
CC := $(TOOLKIT_PREFIX)-gcc
AS := nasm
WARNINGS := -Wall -Wextra -pedantic -Wshadow -Wpointer-arith -Wcast-align \
            -Wwrite-strings -Wmissing-declarations \
            -Wredundant-decls -Wnested-externs -Winline -Wno-long-long \
            -Wuninitialized -Wconversion -Wstrict-prototypes
CFLAGS := $(WARNINGS) -std=gnu99 -ffreestanding -O2 -I$(IDIR)
LFLAGS := -ffreestanding -O2 -nostdlib -lgcc $(WARNINGS)
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

$(ODIR)/shorai.bin: $(OBJFILES)
	$(CC) $(LFLAGS) -T src/linker.ld -o $(ODIR)/shorai.bin $(OBJFILES)

$(ODIR)/%.o: $(SRCDIR)/%.c
	@mkdir -p $(@D)
	$(CC) $(CFLAGS) -c $< -o $@

$(ODIR)/%.o: $(SRCDIR)/%.asm
	@mkdir -p $(@D)
	$(AS) $(ASFLAGS) -o $@ $<

clean:
	-$(RM) $(wildcard $(OBJFILES) shorai.iso $(ODIR)/shorai.bin)
	-$(RM) -r $(ODIR) $(ISODIR)
