from RPLCD.i2c import CharLCD
import spidev
from time import gmtime, strftime, sleep, localtime
import RPi.GPIO as GPIO

# SPI Setup
spi = spidev.SpiDev()

PUSH_BUTTON = 26

lcd = CharLCD(i2c_expander='PCF8574', address=0x27, port=1, cols=16, rows=2, dotsize=8)
lcd.clear()

GPIO.setmode(GPIO.BCM)
GPIO.setup(PUSH_BUTTON, GPIO.IN, pull_up_down=GPIO.PUD_UP)

def read_mcp3008(channel):
    """Read analog value from a specified MCP3008 channel (0-7)."""
    if channel < 0 or channel > 7:
        raise ValueError("Channel must be between 0 and 7")

    spi.open(1, 0)  # Use SPI1, device 0 (CS0 = GPIO 18)
    spi.max_speed_hz = 1000000  # Set SPI speed to 1MHz
    adc = spi.xfer2([1, (8 + channel) << 4, 0])  # Start bit + Single/Diff bit + Channel
    spi.close()  # Close SPI to release CS0

    result = ((adc[1] & 3) << 8) + adc[2]  # Combine the result bytes
    return result

def init_max7219():
    """Initialize the MAX7219 display."""
    lcd.clear()
    lcd.write_string("Init MAX7219...")
    spi.open(1, 1)  # Use SPI1, device 1 (CS1 = GPIO 17)
    spi.max_speed_hz = 1000000  # Set SPI speed to 1MHz
    spi.xfer2([0x09, 0x00])  # No decode mode for 8x8 matrix
    spi.xfer2([0x0A, 0x08])  # Intensity: Medium brightness
    spi.xfer2([0x0B, 0x07])  # Scan limit: Display rows 0-7
    spi.xfer2([0x0C, 0x01])  # Shutdown register: Normal operation
    spi.xfer2([0x0F, 0x00])  # Display test: Off
    spi.close()  # Close SPI to release CS1

def write_max7219(row, value):
    """Write a value to a specific row on the MAX7219 8x8 LED matrix."""
    spi.open(1, 1)  # Use SPI1, device 1 (CS1 = GPIO 17)
    spi.max_speed_hz = 1000000  # Set SPI speed to 1MHz
    spi.xfer2([row, value])
    spi.close()  # Close SPI to release CS1

try:
    # Initialize MAX7219
    init_max7219()
    lcd.clear()
    frame = 0

    # Example usage
    while True:
        
        frame += 1
        startTime = localtime()

        # Read analog value from channel 0 of MCP3008
        analog_value = read_mcp3008(0)
        analog_value2 = read_mcp3008(1)
        #lcd.clear()
        
		# read button state
        button_state = GPIO.input(PUSH_BUTTON)
       

		# get the value from 0 to 1023 to 1 to 8
        analog_value_clipped = int(analog_value / 128) + 1
        analog_value2_clipped = int(analog_value2 / 128) + 1
        
		# display over 2 lines of 8 characters
        if frame % 60 == 0:
            print("Analog 0: {0:04d}".format(analog_value))
            lcd.cursor_pos = (0, 0)
            lcd.write_string("Analog 0: {0:04d}".format(analog_value))
            lcd.cursor_pos = (1, 0)
            lcd.write_string("Analog 1: {0:04d}".format(analog_value2))
            frame = 0
        
        if button_state == 0:
            # all leds on
            for i in range(1, 9):
                write_max7219(i, 255)
        else:
		    # black screen
            for i in range(1, 9):
                write_max7219(i, 0)
 				# Display a point in the matrix display, x being analog_value and y being analog_value2
                write_max7219(analog_value2_clipped, 1 << (9-analog_value_clipped-1))

		# Wait for the next frame to display
        endTime = localtime()
        elapsedTime = endTime.tm_sec - startTime.tm_sec
        
        hz = 60
        
        sleepTime = 1/hz - elapsedTime
        if sleepTime > 0:
            sleep(sleepTime)

except KeyboardInterrupt:
    print("Exiting program...")

finally:
    lcd.clear()
    lcd.close(clear=True)
