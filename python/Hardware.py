import random
import spidev
import math
import RPi.GPIO as GPIO

buttonsGPIO = [22, 23, 24, 25, 26, 27]
buttonsGPIO.reverse()

def init_hardware():
    GPIO.setmode(GPIO.BCM)
    for button in buttonsGPIO:
        GPIO.setup(button, GPIO.IN, pull_up_down=GPIO.PUD_UP)

def read_joystick(index):
    return 0

def read_slider(index):
    # Read the slider value from the gpio
    spi = spidev.SpiDev()

    value = read_mcp3008(index, spi)
    return value

def read_rotary(index):
    return 0

def read_pot(index):
    return 0

def read_button(index):
    # Read the button value from the gpio state (0 = pressed, 1 = released)
    value = GPIO.input(buttonsGPIO[index]) == 0
    return value

def read_mcp3008(channel, spi):
    """Read analog value from a specified MCP3008 channel (0-7)."""
    if channel < 0 or channel > 7:
        raise ValueError("Channel must be between 0 and 7")

    spi.open(1, 0)  # Use SPI1, device 0 (CS0 = GPIO 18)
    spi.max_speed_hz = 1000000  # Set SPI speed to 1MHz
    adc = spi.xfer2([1, (8 + channel) << 4, 0])  # Start bit + Single/Diff bit + Channel
    spi.close()  # Close SPI to release CS0

    result = ((adc[1] & 3) << 8) + adc[2]  # Combine the result bytes
    
    # resize the value from 0-1023 to 0-255
    # result = adc_log_to_linear(result)
    result = int(result / 4)
    return result

def adc_log_to_linear(adc_value):
    # Invert the logarithmic mapping:
    linear_value = math.exp((adc_value / 1023) * math.log(256)) - 1
    # Clamp the value to integer range 0-255
    return int(round(linear_value))