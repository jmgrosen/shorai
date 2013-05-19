
#include <stddef.h>
#include <stdint.h>

void term_init(void);
void term_setcolor(uint8_t color);
void term_putentryat(char c, uint8_t color, size_t x, size_t y);
void term_scroll(void);
void term_putchar(char c);
void term_writestring(const char* s);
