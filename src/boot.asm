; Copyright (c) 2013 John Grosen
; Licensed under the terms described in the LICENSE file in the root of this repository


MBALIGN     equ  1<<0
MEMINFO     equ  1<<1
FLAGS       equ  MBALIGN | MEMINFO
MAGIC       equ  0x1BADB002
CHECKSUM    equ -(MAGIC + FLAGS)


section .multiboot
align 4
	dd MAGIC
	dd FLAGS
	dd CHECKSUM


global _GLOBAL_OFFSET_TABLE_
global __morestack
global abort
global memcmp
global memcpy
global malloc
global free
global _start

extern kernel_main

section .bootstrap_stack
align 4
stack_bottom:
times 16384 db 0
stack_top:

section .text
_start:

	mov esp, stack_top

        mov [gs:0x30], dword 0

	mov edi, 0xb8000
	mov ecx, 80*25*2
	mov al, 1
	rep stosb

        call kernel_main
        jmp $

_GLOBAL_OFFSET_TABLE_:

__morestack:

abort:
        jmp $

memcmp:
        jmp $

memcpy:
        jmp $

malloc:
        jmp $

free:
        jmp $
