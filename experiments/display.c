#include <pigpio.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <time.h>
#include "i2clcd.h"

#define SPI_CHANNEL_MCP3008 0
#define SPI_CHANNEL_MAX7219 1
#define SPI_SPEED 1000000 // 1 MHz
#define PUSH_BUTTON 26
#define I2C_ADDRESS 0x27

int lcd_handle;

int spi_handle_mcp3008 = -1; // SPI handle for MCP3008
int spi_handle_max7219 = -1; // SPI handle for MAX7219

void init_lcd()
{
    // Open I2C connection
    int lcd_handle = i2cOpen(1, I2C_ADDRESS, 0);
    if (lcd_handle < 0)
    {
        fprintf(stderr, "Failed to open I2C connection\n");
        gpioTerminate();
    }

    // Initialize the LCD
    lcd_init(lcd_handle);

    // Write text to the LCD
    write_line(lcd_handle, 0, "Hello, World!");
    write_line(lcd_handle, 1, "Pigpio LCD Test");
}

// Initialize SPI for both devices
int init_spi()
{
    // Open SPI channel for MCP3008
    spi_handle_mcp3008 = spiOpen(SPI_CHANNEL_MCP3008, SPI_SPEED, 256);
    if (spi_handle_mcp3008 < 0)
    {
        printf("Failed to open SPI channel for MCP3008: %d\n", spi_handle_mcp3008);
        return -1;
    }
    else
    {
        printf("SPI channel for MCP3008 opened: %d\n", spi_handle_mcp3008);
    }

    // Open SPI channel for MAX7219
    spi_handle_max7219 = spiOpen(SPI_CHANNEL_MAX7219, SPI_SPEED, 256);
    if (spi_handle_max7219 < 0)
    {
        printf("Failed to open SPI channel for MAX7219: %d\n", spi_handle_max7219);
        return -1;
    }
    else
    {
        printf("SPI channel for MAX7219 opened: %d\n", spi_handle_max7219);
    }

    return 0;
}

// Close both SPI handles
void close_spi()
{
    if (spi_handle_mcp3008 >= 0)
    {
        spiClose(spi_handle_mcp3008);
    }
    if (spi_handle_max7219 >= 0)
    {
        spiClose(spi_handle_max7219);
    }
}

// Read analog value from MCP3008
// Read analog value from MCP3008
int read_mcp3008(int channel)
{
    if (channel < 0 || channel > 7)
    {
        fprintf(stderr, "Channel must be between 0 and 7\n");
        return -1;
    }

    char tx[3] = {1, (8 + channel) << 4, 0};
    char rx[3] = {0};
    spiXfer(spi_handle_mcp3008, tx, rx, 3);

    int result = ((rx[1] & 3) << 8) + rx[2];

    // printf("Result: %d\n", result);
    return result;
}

// Initialize MAX7219
void init_max7219()
{
    char init_data[][2] = {
        {0x09, 0x00}, // Decode mode: None
        {0x0A, 0x08}, // Intensity: Medium brightness
        {0x0B, 0x07}, // Scan limit: Display rows 0-7
        {0x0C, 0x01}, // Shutdown register: Normal operation
        {0x0F, 0x00}  // Display test: Off
    };

    for (int i = 0; i < 5; i++)
    {
        spiWrite(spi_handle_max7219, init_data[i], 2);
    }

    printf("MAX7219 initialized\n");
}

// Write data to MAX7219
void write_max7219(int row, int value)
{
    char buffer[2] = {row, value};
    spiWrite(spi_handle_max7219, buffer, 2);
}

int main()
{
    if (gpioInitialise() < 0)
    {
        fprintf(stderr, "Failed to initialize GPIO\n");
        return 1;
    }

    if (init_spi() < 0)
    {
        gpioTerminate();
        return 1;
    }

    // Setup push button
    gpioSetMode(PUSH_BUTTON, PI_INPUT);
    gpioSetPullUpDown(PUSH_BUTTON, PI_PUD_UP);

    // Initialize MAX7219
    init_max7219();

    int frame = 0;

    init_lcd(); // Initialize the LCD
    while (1)
    {
        frame++;
        time_t startTime = time(NULL);

        // Read analog values
        int analog_value = read_mcp3008(0);
        int analog_value2 = read_mcp3008(1);

        // Read button state
        int button_state = gpioRead(PUSH_BUTTON);

        // Clip analog values
        int analog_value_clipped = analog_value / 128 + 1;
        int analog_value2_clipped = analog_value2 / 128 + 1;

        // char buffer[100]; // Create a buffer to hold the formatted string
        // sprintf(buffer, "Frame: %d, Analog 1: %d, Analog 2: %d, Button: %d\n", frame, analog_value, analog_value2, button_state);
        // printf("%s", buffer);

        // Update MAX7219
        if (button_state == 0)
        {
            for (int i = 1; i <= 8; i++)
            {
                write_max7219(i, 255); // All LEDs on
            }
        }
        else
        {
            for (int i = 1; i <= 8; i++)
            {
                write_max7219(i, 0); // Black screen
            }
            write_max7219(analog_value2_clipped, 1 << (8 - analog_value_clipped));
        }

        // Wait for the next frame
        time_t endTime = time(NULL);
        double elapsedTime = difftime(endTime, startTime);
        double sleepTime = 1.0 / 60.0 - elapsedTime;

        if (sleepTime > 0)
        {
            usleep(sleepTime * 1000000);
        }
    }

    close_spi();
    gpioTerminate();

    return 0;
}