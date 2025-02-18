import random
from Mcp3008 import read_mcp3008
import spidev

def read_joystick(index):
    return random.randint(1, 6)

def read_slider(index):
    # Read the slider value from the gpio
    spi = spidev.SpiDev()

    value = read_mcp3008(index, spi)
    return value

def read_rotary(index):
    return random.randint(1, 6)

def read_pot(index):
    return random.randint(1, 6)

def read_button(index):
    return random.randint(0, 1)