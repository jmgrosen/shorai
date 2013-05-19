
SRCDIR := src
IDIR := include
ODIR := obj
ISODIR := isodir

SRCFILES := $(shell find $(SRCDIR) -type f -name "*.[cs]")
CFILES := $(filter %.c,$(SRCFILES))
SFILES := $(filter %.s,$(SRCFILES))
OBJFILES := $(patsubst $(SRCDIR)/%.s,$(ODIR)/%.o,$(SFILES)) \
	    $(patsubst $(SRCDIR)/%.c,$(ODIR)/%.o,$(CFILES))

TOOLKIT_PREFIX := i586-elf-
CC := $(TOOLKIT_PREFIX)gcc
AS := $(TOOLKIT_PREFIX)as
WARNINGS := -Wall -Wextra -pedantic -Wshadow -Wpointer-arith -Wcast-align \
            -Wwrite-strings -Wmissing-declarations \
            -Wredundant-decls -Wnested-externs -Winline -Wno-long-long \
            -Wuninitialized -Wconversion -Wstrict-prototypes
CFLAGS := -std=gnu99 -ffreestanding -O2 -I$(IDIR)
LFLAGS := -ffreestanding -O2 -nostdlib -lgcc $(WARNINGS)

.PHONY: all run clean

all: shorai.iso

run: shorai.iso
	qemu-system-i386 -cdrom shorai.iso

shorai.iso: $(ODIR)/shorai.bin
	mkdir -p $(ISODIR)
	mkdir -p $(ISODIR)/boot
	cp $(ODIR)/shorai.bin $(ISODIR)/boot/shorai.bin
	mkdir -p $(ISODIR)/boot/grub
	cp src/grub.cfg $(ISODIR)/boot/grub/grub.cfg
	grub-mkrescue -o shorai.iso $(ISODIR)

$(ODIR)/shorai.bin: $(OBJFILES)
	$(CC) $(LFLAGS) -T src/linker.ld -o $(ODIR)/shorai.bin $(OBJFILES)

$(ODIR)/%.o: $(SRCDIR)/%.c
	@mkdir -p $(@D)
	$(CC) $(CFLAGS) -c $< -o $@

$(ODIR)/%.o: $(SRCDIR)/%.s
	@mkdir -p $(@D)
	$(AS) -o $@ $<

clean:
	-$(RM) $(wildcard $(OBJFILES) shorai.iso $(ODIR)/shorai.bin)
	-$(RM) -r $(ODIR) $(ISODIR)
