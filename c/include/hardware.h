#ifndef HARDWARE_H
#define HARDWARE_H

#define NUM_BUTTONS 8

// Extern declaration for the button GPIO pins
extern int buttonsGPIO[NUM_BUTTONS];

// Initialize hardware interfaces (GPIO, SPI, I2C)
void init_hardware(void);

// Hardware reading functions
int read_joystick(int index);
int read_slider_middle(int index);
int read_slider(int index);
int read_rotary(int index);
int read_pot(int index);
int read_button(int index);
int read_mcp3008(int channel);
int adc_log_to_linear(int adc_value);
// Returns pointer to a static array of two encoder values; or NULL on error.
int *read_encoders(void);

#endif // HARDWARE_H