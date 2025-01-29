#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include <linux/i2c-dev.h>
#include <sys/ioctl.h>
#include <string.h>

#define I2C_DEVICE "/dev/i2c-1"  // I2C bus device
#define ARDUINO_I2C_ADDRESS 0x08 // Arduino I2C address
#define NUM_ENCODERS 4
#define BUFFER_SIZE (NUM_ENCODERS * sizeof(int32_t)) // 4 * 4 = 16 bytes

// Function to read encoder values from Arduino
int read_encoders(int file, int32_t *encoder_values)
{
    uint8_t buffer[BUFFER_SIZE];

    // Read data from Arduino
    if (read(file, buffer, BUFFER_SIZE) != BUFFER_SIZE)
    {
        perror("Failed to read from I2C device");
        return -1;
    }

    // Convert raw bytes to integers (little-endian format)
    for (int i = 0; i < NUM_ENCODERS; i++)
    {
        memcpy(&encoder_values[i], &buffer[i * sizeof(int32_t)], sizeof(int32_t));
    }

    return 0;
}

int main()
{
    int file;
    int32_t encoder_values[NUM_ENCODERS] = {0}; // Current encoder values
    int32_t last_values[NUM_ENCODERS] = {0};    // Store last known values

    // Open the I2C device
    if ((file = open(I2C_DEVICE, O_RDONLY)) < 0)
    {
        perror("Failed to open I2C bus");
        return 1;
    }

    // Set the I2C slave address
    if (ioctl(file, I2C_SLAVE, ARDUINO_I2C_ADDRESS) < 0)
    {
        perror("Failed to set I2C slave address");
        close(file);
        return 1;
    }

    while (1)
    {
        // Read encoder values
        if (read_encoders(file, encoder_values) == 0)
        {
            // Check if any value has changed
            int changed = 0;
            for (int i = 0; i < NUM_ENCODERS; i++)
            {
                if (encoder_values[i] != last_values[i])
                {
                    changed = 1;
                    break;
                }
            }

            // Print values only if they have changed
            if (changed)
            {
                printf("Encoder Values: %d, %d, %d, %d\n",
                       encoder_values[0], encoder_values[1],
                       encoder_values[2], encoder_values[3]);

                // Update last known values
                memcpy(last_values, encoder_values, sizeof(last_values));
            }
        }
        usleep(10000); // 10ms delay
    }

    close(file);
    return 0;
}
