#include <hardware.h>
#include <wiringPi.h>
#include <wiringPiSPI.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <fcntl.h>
#include <unistd.h>
#include <sys/ioctl.h>
#include <linux/i2c-dev.h>
#include <math.h>

// SPI settings
#define SPI_CHANNEL 0
#define SPI_SPEED 1000000

// I2C settings
#define I2C_DEVICE "/dev/i2c-1"
#define ARDUINO_I2C_ADDRESS 0x08

// Number of encoders to read (2 as in the Python code)
#define NUM_ENCODERS 2

// Button GPIO pins (using BCM numbering)
int buttonsGPIO[NUM_BUTTONS] = {22, 23, 24, 25, 12, 4, 5, 6};

// Global file descriptor for the I2C bus
static int i2c_fd = -1;

void init_hardware(void)
{
    // Initialize WiringPi with BCM GPIO numbering
    if (wiringPiSetupGpio() == -1)
    {
        fprintf(stderr, "Failed to initialize WiringPi\n");
        exit(1);
    }

    // Setup buttons
    for (int i = 0; i < NUM_BUTTONS; i++)
    {
        pinMode(buttonsGPIO[i], INPUT);
        pullUpDnControl(buttonsGPIO[i], PUD_UP);
    }

    // Setup SPI channel for the MCP3008 sensor
    if (wiringPiSPISetup(SPI_CHANNEL, SPI_SPEED) == -1)
    {
        fprintf(stderr, "SPI Setup failed\n");
        exit(1);
    }

    // Open the I2C device and set the Arduino address
    i2c_fd = open(I2C_DEVICE, O_RDWR);
    if (i2c_fd < 0)
    {
        perror("I2C: Failed to open device");
    }
    else
    {
        if (ioctl(i2c_fd, I2C_SLAVE, ARDUINO_I2C_ADDRESS) < 0)
        {
            perror("I2C: Failed to select device");
        }
    }

    printf("Hardware initialized\n");
}

int read_joystick(int index)
{
    // Not implemented; return default value
    return 0;
}

int read_mcp3008(int channel)
{
    if (channel < 0 || channel > 7)
    {
        fprintf(stderr, "Channel must be between 0 and 7\n");
        return -1;
    }

    // Prepare the data array for SPI transfer.
    // We send 3 bytes: start bit, configuration byte, dummy byte.
    unsigned char data[3];
    data[0] = 1;                  // Start bit
    data[1] = (8 + channel) << 4; // SGL/DIF bit (single-ended) and channel selection
    data[2] = 0;                  // Dummy byte

    // Perform SPI transfer using WiringPi (data is updated in place)
    wiringPiSPIDataRW(SPI_CHANNEL, data, 3);

    // Combine the returned bytes: bits 0-1 of data[1] and all of data[2]
    int adc = ((data[1] & 3) << 8) | data[2];

    // Scale the 10-bit result (0-1023) to an 8-bit value (0-255)
    int result = adc / 4;
    return result;
}

int read_slider_middle(int index)
{
    int val = read_mcp3008(index);
    int result = (val - 128) & 0xFF;
    return result;
}

int read_slider(int index)
{
    return read_mcp3008(index);
}

int read_rotary(int index)
{
    // Store the last values in a static array
    static int lastValues[NUM_ENCODERS] = {0, 0};
    int *enc = read_encoders();
    int delta = 0;
    if (enc != NULL)
    {
        delta = enc[index] - lastValues[index];
        lastValues[index] = enc[index];
    }
    return delta;
}

int read_pot(int index)
{
    // Not implemented; return default value
    return 0;
}

int read_button(int index)
{
    // Returns 1 if button is pressed (GPIO is LOW), otherwise 0.
    int pressed = (digitalRead(buttonsGPIO[index]) == LOW) ? 1 : 0;
    return pressed;
}

int adc_log_to_linear(int adc_value)
{
    double linear_value = exp(((double)adc_value / 1023.0) * log(256.0)) - 1.0;
    int result = (int)round(linear_value);
    if (result < 0)
        result = 0;
    if (result > 255)
        result = 255;
    return result;
}

int *read_encoders(void)
{
    static int values[NUM_ENCODERS] = {0, 0};
    if (i2c_fd < 0)
    {
        fprintf(stderr, "I2C not initialized\n");
        return NULL;
    }

    // Read NUM_ENCODERS bytes from the I2C bus.
    unsigned char buf[NUM_ENCODERS];
    int bytes = read(i2c_fd, buf, NUM_ENCODERS);
    if (bytes != NUM_ENCODERS)
    {
        fprintf(stderr, "Error reading from I2C: expected %d bytes, got %d\n", NUM_ENCODERS, bytes);
        return NULL;
    }

    // Unpack the two bytes as unsigned integers.
    values[0] = buf[0];
    values[1] = buf[1];
    return values;
}