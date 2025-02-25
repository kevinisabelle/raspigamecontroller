import spidev
import math
import RPi.GPIO as GPIO
import smbus2
import time
import struct

buttonsGPIO = [22, 23, 24, 25, 26, 27, 12, 6]
# buttonsGPIO.reverse()

# Initialize I2C bus
bus = smbus2.SMBus(1)
print("I2C bus initialized")

def init_hardware():
    GPIO.setmode(GPIO.BCM)
    for button in buttonsGPIO:
        GPIO.setup(button, GPIO.IN, pull_up_down=GPIO.PUD_UP)

def read_joystick(index):
    return 0

def read_slider_middle(index):
    # Read the slider value from the gpio
    spi = spidev.SpiDev()

    value = (read_mcp3008(index, spi) - 128) & 0xFF
    return value

def read_slider(index):
    # Read the slider value from the gpio
    spi = spidev.SpiDev()

    value = read_mcp3008(index, spi)
    return value

encoderLastValues = [0, 0]

def read_rotary(index):
    encoders = read_encoders()
    delta = 0

    if encoders is not None:
        delta = encoders[index] - encoderLastValues[index]
        encoderLastValues[index] = encoders[index]

    return delta

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


ARDUINO_I2C_ADDRESS = 0x08

# Number of encoders
NUM_ENCODERS = 2

def read_encoders():
    try:
        # Request 4 long integers (4 bytes each) from Arduino
        raw_data = bus.read_i2c_block_data(ARDUINO_I2C_ADDRESS, 0, NUM_ENCODERS)
        
        raw_bytes = bytes(raw_data)
        # print (raw_bytes)
        encoder_values = struct.unpack('<BB', raw_bytes)
        # print(encoder_values)
        return encoder_values
    except Exception as e:
        print(f"Error reading from Arduino: {e}")
        return None
