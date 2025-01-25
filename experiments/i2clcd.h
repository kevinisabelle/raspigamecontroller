#ifndef I2CLCD_H
#define I2CLCD_H

#include <pigpio.h>

// Define LCD Commands
#define LCD_CLEAR 0x01
#define LCD_RETURN_HOME 0x02
#define LCD_ENTRY_MODE 0x04
#define LCD_DISPLAY_CONTROL 0x08
#define LCD_CURSOR_SHIFT 0x10
#define LCD_FUNCTION_SET 0x20
#define LCD_SET_CGRAM_ADDR 0x40
#define LCD_SET_DDRAM_ADDR 0x80

// Flags for Display Entry Mode
#define LCD_ENTRY_RIGHT 0x00
#define LCD_ENTRY_LEFT 0x02
#define LCD_ENTRY_SHIFT_INCREMENT 0x01
#define LCD_ENTRY_SHIFT_DECREMENT 0x00

// Flags for Display Control
#define LCD_DISPLAY_ON 0x04
#define LCD_DISPLAY_OFF 0x00
#define LCD_CURSOR_ON 0x02
#define LCD_CURSOR_OFF 0x00
#define LCD_BLINK_ON 0x01
#define LCD_BLINK_OFF 0x00

// LCD Backlight
#define LCD_BACKLIGHT 0x08
#define LCD_NOBACKLIGHT 0x00

// Control Bits
#define ENABLE 0x04          // Enable bit
#define REGISTER_SELECT 0x01 // Register Select bit (0 = command, 1 = data)

// Function Prototypes
void lcd_init(int handle);
void lcd_clear(int handle);
void lcd_set_cursor(int handle, int col, int row);
void write_line(int handle, int line, const char *text);

#endif // I2CLCD_H