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
    result = int(result / 4)
    return result