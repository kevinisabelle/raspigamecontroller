import smbus2
import time
import struct

# I2C address of the Arduino
ARDUINO_I2C_ADDRESS = 0x08

# Number of encoders
NUM_ENCODERS = 4

# Initialize I2C bus
bus = smbus2.SMBus(1)

def read_encoders():
    try:
        # Request 4 long integers (4 bytes each) from Arduino
        raw_data = bus.read_i2c_block_data(ARDUINO_I2C_ADDRESS, 0, NUM_ENCODERS * 4)
        
        # Unpack binary data into integers
        encoder_values = struct.unpack('<llll', bytes(raw_data))
        return encoder_values
    except Exception as e:
        print(f"Error reading from Arduino: {e}")
        return None

if __name__ == "__main__":
    while True:
        encoders = read_encoders()
        if encoders:
            print(f"Encoder Values: {encoders}")
        time.sleep(0.1)
