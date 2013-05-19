
#include <stdint.h>
#include <stddef.h>
#include <string.h>

#include <vga.h>
#include <term.h>

size_t term_row;
size_t term_column;
uint8_t term_color;
uint16_t* term_buffer;

void term_init() {
  term_row = 0;
  term_column = 0;
  term_color = vga_make_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
  term_buffer = (uint16_t*) 0xB8000;
  for (size_t y = 0; y < VGA_HEIGHT; y++) {
    for (size_t x = 0; x < VGA_WIDTH; x++) {
      const size_t index = y * VGA_WIDTH + x;
      term_buffer[index] = vga_make_entry(' ', term_color);
    }
  }
}

void term_setcolor(uint8_t color) {
  term_color = color;
}

void term_putentryat(char c, uint8_t color, size_t x, size_t y) {
  const size_t index = y * VGA_WIDTH + x;
  term_buffer[index] = vga_make_entry(c, color);
}

void term_scroll() {
  uint16_t* p = &term_buffer[VGA_WIDTH];
  uint16_t* limit = &term_buffer[VGA_HEIGHT * VGA_WIDTH];
  for (; p < limit; p++) {
    p[-VGA_WIDTH] = *p;
  }
}

void term_putchar(char c) {
  if (c == '\n') {
    term_column = 0;
    if (++term_row == VGA_HEIGHT) {
      term_row--;
      term_scroll();
    }
    return;
  }
  term_putentryat(c, term_color, term_column, term_row);
  if (++term_column == VGA_WIDTH) {
    term_column = 0;
    if (++term_row == VGA_HEIGHT) {
      term_row--;
      term_scroll();
    }
  }
}

void term_writestring(const char* data) {
  size_t datalen = strlen(data);
  for (size_t i = 0; i < datalen; i++) {
    term_putchar(data[i]);
  }
}
