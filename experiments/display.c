#include <pthread.h>
#include <pigpio.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <time.h>
#include "lcd1602.h"

#define SPI_CHANNEL_MCP3008 0
#define SPI_CHANNEL_MAX7219 1
#define SPI_SPEED 1000000 // 1 MHz
#define PUSH_BUTTON 26

int spi_handle_mcp3008 = -1; // SPI handle for MCP3008
int spi_handle_max7219 = -1; // SPI handle for MAX7219

pthread_mutex_t data_mutex = PTHREAD_MUTEX_INITIALIZER;
int analog_value = 0;
int analog_value2 = 0;

// Initialize SPI for both devices
int init_spi()
{
    spi_handle_mcp3008 = spiOpen(SPI_CHANNEL_MCP3008, SPI_SPEED, 256);
    if (spi_handle_mcp3008 < 0)
    {
        printf("Failed to open SPI channel for MCP3008: %d\n", spi_handle_mcp3008);
        return -1;
    }

    spi_handle_max7219 = spiOpen(SPI_CHANNEL_MAX7219, SPI_SPEED, 256);
    if (spi_handle_max7219 < 0)
    {
        printf("Failed to open SPI channel for MAX7219: %d\n", spi_handle_max7219);
        return -1;
    }

    return 0;
}

void close_spi()
{
    if (spi_handle_mcp3008 >= 0)
        spiClose(spi_handle_mcp3008);
    if (spi_handle_max7219 >= 0)
        spiClose(spi_handle_max7219);
}

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

    return ((rx[1] & 3) << 8) + rx[2];
}

void init_max7219()
{
    char init_data[][2] = {
        {0x09, 0x00},
        {0x0A, 0x08},
        {0x0B, 0x07},
        {0x0C, 0x01},
        {0x0F, 0x00}};

    for (int i = 0; i < 5; i++)
    {
        spiWrite(spi_handle_max7219, init_data[i], 2);
    }
    printf("MAX7219 initialized\n");
}

void write_max7219(int row, int value)
{
    char buffer[2] = {row, value};
    spiWrite(spi_handle_max7219, buffer, 2);
}

// Thread for LCD refresh
void *lcd_refresh_thread(void *arg)
{
    while (1)
    {
        pthread_mutex_lock(&data_mutex);
        int local_analog_value = analog_value;
        int local_analog_value2 = analog_value2;
        pthread_mutex_unlock(&data_mutex);

        lcd1602SetCursor(0, 0);
        char buffer[100];
        sprintf(buffer, "Analog 1: %d   ", local_analog_value);
        lcd1602WriteString(buffer);

        lcd1602SetCursor(0, 1);
        sprintf(buffer, "Analog 2: %d   ", local_analog_value2);
        lcd1602WriteString(buffer);

        usleep(100000); // Refresh every 100ms
    }
    return NULL;
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

    int rc = lcd1602Init(1, 0x27);
    if (rc)
    {
        printf("LCD initialization failed; aborting...\n");
        return 0;
    }
    lcd1602WriteString("   Gamepad      ");
    lcd1602SetCursor(0, 1);
    lcd1602WriteString("   v0.1         ");
    lcd1602Control(1, 0, 1);

    gpioSetMode(PUSH_BUTTON, PI_INPUT);
    gpioSetPullUpDown(PUSH_BUTTON, PI_PUD_UP);
    init_max7219();

    pthread_t lcd_thread;
    pthread_create(&lcd_thread, NULL, lcd_refresh_thread, NULL);

    while (1)
    {
        int temp_analog_value = read_mcp3008(0);
        int temp_analog_value2 = read_mcp3008(1);
        int button_state = gpioRead(PUSH_BUTTON);

        pthread_mutex_lock(&data_mutex);
        analog_value = temp_analog_value;
        analog_value2 = temp_analog_value2;
        pthread_mutex_unlock(&data_mutex);

        if (button_state == 0)
        {
            for (int i = 1; i <= 8; i++)
            {
                write_max7219(i, 255);
            }
        }
        else
        {
            for (int i = 1; i <= 8; i++)
            {
                write_max7219(i, 0);
            }
            int analog_value_clipped = temp_analog_value / 128 + 1;
            int analog_value2_clipped = temp_analog_value2 / 128 + 1;
            write_max7219(analog_value2_clipped, 1 << (8 - analog_value_clipped));
        }

        usleep(16667); // ~60 FPS loop
    }

    close_spi();
    lcd1602Shutdown();
    gpioTerminate();

    return 0;
}
