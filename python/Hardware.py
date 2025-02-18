import random

def read_joystick(index):
    return random.randint(1, 6)

def read_slider(index):
    return random.randint(1, 6)

def read_rotary(index):
    return random.randint(1, 6)

def read_pot(index):
    return random.randint(1, 6)

def read_button(index):
    return random.randint(0, 1)