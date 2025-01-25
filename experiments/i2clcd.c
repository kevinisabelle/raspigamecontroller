// i2clcd.c
#include "i2clcd.h"
#include <unistd.h>
#include <string.h>
#include <stdio.h>
#include <stdlib.h>

static int backlight_state = LCD_BACKLIGHT;

static void lcd_write_byte(int handle, int data)
{
    // Write high nibble
    i2cWriteByte(handle, data | ENABLE | backlight_state);
    usleep(1);
    i2cWriteByte(handle, (data & ~ENABLE) | backlight_state);

    // Write low nibble
    i2cWriteByte(handle, ((data << 4) & 0xF0) | ENABLE | backlight_state);
    usleep(1);
    i2cWriteByte(handle, ((data << 4) & 0xF0) | backlight_state);
}

void lcd_send_command(int handle, int command)
{
    lcd_write_byte(handle, command & ~REGISTER_SELECT);
    usleep(100);
}

void lcd_send_data(int handle, int data)
{
    lcd_write_byte(handle, data | REGISTER_SELECT);
    usleep(100);
}

void lcd_clear(int handle)
{
    lcd_send_command(handle, LCD_CLEAR);
    usleep(2000);
}

void lcd_set_cursor(int handle, int col, int row)
{
    int row_offsets[] = {0x00, 0x40, 0x14, 0x54};
    lcd_send_command(handle, LCD_SET_DDRAM_ADDR | (col + row_offsets[row]));
}

void lcd_init(int handle)
{
    usleep(50000); // Wait for LCD to power up

    // Send initialization commands
    lcd_send_command(handle, 0x03);
    usleep(5000);
    lcd_send_command(handle, 0x03);
    usleep(5000);
    lcd_send_command(handle, 0x03);
    usleep(5000);
    lcd_send_command(handle, 0x02); // Set to 4-bit mode

    // Configure display
    lcd_send_command(handle, LCD_FUNCTION_SET | 0x08); // 2 lines, 5x8 dots
    lcd_send_command(handle, LCD_DISPLAY_CONTROL | LCD_DISPLAY_ON | LCD_CURSOR_OFF | LCD_BLINK_OFF);
    lcd_send_command(handle, LCD_CLEAR);
    lcd_send_command(handle, LCD_ENTRY_MODE | LCD_ENTRY_LEFT);
    usleep(2000);
}

void write_line(int handle, int line, const char *text)
{
    if (line < 0 || line > 1)
    {
        fprintf(stderr, "Invalid line number: %d\n", line);
        return;
    }

    lcd_set_cursor(handle, 0, line);

    // Write text to the LCD
    for (int i = 0; i < strlen(text) && i < 16; i++)
    {
        lcd_send_data(handle, text[i]);
    }

    // Fill the rest of the line with spaces
    for (int i = strlen(text); i < 16; i++)
    {
        lcd_send_data(handle, ' ');
    }
}
