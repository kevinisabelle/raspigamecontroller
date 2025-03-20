// rust
use std::io;

use once_cell::sync::Lazy;
use rppal::gpio::{Gpio, InputPin, Level};
use rppal::i2c::I2c;
use spidev::{SpiModeFlags, Spidev, SpidevOptions, SpidevTransfer};
use std::f64;
use std::sync::Mutex;

// Button GPIO pins.
pub const BUTTONS_GPIO: [u8; 8] = [22, 23, 24, 25, 12, 4, 5, 6];

// Global encoder last values.
static ENCODER_LAST_VALUES: Lazy<Mutex<[i32; 2]>> = Lazy::new(|| Mutex::new([0, 0]));

// I2C address for Arduino.
pub const ARDUINO_I2C_ADDRESS: u16 = 0x08;

// Number of encoders.
pub const NUM_ENCODERS: usize = 2;

/// Initialize hardware by setting up GPIO pins for buttons.
pub fn init_hardware() -> Result<Vec<InputPin>, io::Error> {
    let gpio = Gpio::new().unwrap();
    let mut pins = Vec::new();
    for &btn in BUTTONS_GPIO.iter() {
        let pin = gpio.get(btn).unwrap().into_input_pulldown();
        // As Python uses pull_up, here we invert logic if needed.
        pins.push(pin);
    }
    println!("I2C bus initialized");
    Ok(pins)
}

/// Returns a dummy joystick reading.
pub fn read_joystick(_index: u8) -> u8 {
    0
}

/// Reads the middle slider value with an offset.
/// It creates a SPI device, reads the MCP3008 channel,
/// subtracts 128 and masks to 8 bits.
pub fn read_slider_middle(index: u8) -> io::Result<u8> {
    let raw = read_mcp3008(index)?;
    let value = ((raw as i32 - 128).max(0) & 0xFF) as u8;
    Ok(value)
}

/// Reads the slider value from the MCP3008 channel.
pub fn read_slider(index: u8) -> io::Result<u8> {
    let raw = read_mcp3008(index)?;
    Ok(raw as u8)
}

/// Returns a dummy potentiometer reading.
pub fn read_pot(_index: usize) -> u8 {
    0
}

/// Reads the state of the button at the given index.
/// Returns true if pressed.
pub fn read_button(index: usize) -> io::Result<bool> {
    let gpio = Gpio::new().unwrap();
    let pin = gpio.get(BUTTONS_GPIO[index]).unwrap().into_input_pulldown();
    // In Python a 0 equals pressed.
    Ok(pin.read() == Level::Low)
}

/// Reads an analog value from a specified MCP3008 channel (0-7) via SPI.
pub fn read_mcp3008(channel: u8) -> io::Result<u16> {
    if channel > 7 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Channel must be between 0 and 7",
        ));
    }
    let mut spi = Spidev::open("/dev/spidev1.0")?;
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(1_000_000)
        .mode(SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&options)?;

    // Prepare transmission: start bit, configuration, and dummy byte.
    let mut tx_buf = [1u8, (8 + channel) << 4, 0u8];
    let mut rx_buf = tx_buf;
    {
        let mut transfer = SpidevTransfer::read_write(&mut rx_buf, &mut tx_buf);
        spi.transfer(&mut transfer)?;
    }
    // Combine bytes.
    let result = (((rx_buf[1] & 3) as u16) << 8) | (rx_buf[2] as u16);
    // Resize 10-bit (0-1023) value to 8-bit (0-255).
    Ok(result / 4)
}

/// Inverts logarithmic mapping from ADC value to a linear 0-255 scale.
pub fn adc_log_to_linear(adc_value: u16) -> u8 {
    let adc_value = adc_value as f64;
    let linear_value = f64::exp((adc_value / 1023.0) * f64::ln(256.0)) - 1.0;
    linear_value.round().min(255.0).max(0.0) as u8
}

/// Reads encoder values via I²C from the Arduino.
/// Returns a tuple with two values on success or None if an error occurred.
pub fn read_encoders() -> Option<(u8, u8)> {
    let mut i2c = match I2c::new() {
        Ok(i2c) => i2c,
        Err(e) => {
            eprintln!("Error initializing I2C: {}", e);
            return None;
        }
    };
    if let Err(e) = i2c.set_slave_address(ARDUINO_I2C_ADDRESS as u16) {
        eprintln!("Error setting I2C address: {}", e);
        return None;
    }
    // In Python, NUM_ENCODERS bytes are read. Here we expect two bytes.
    let mut buffer = [0u8; NUM_ENCODERS];
    match i2c.read(&mut buffer) {
        Ok(_) => Some((buffer[0], buffer[1])),
        Err(e) => {
            eprintln!("Error reading from Arduino: {}", e);
            None
        }
    }
}

/// Reads the rotary encoder delta for the given index.
/// It reads new encoder values via I²C, computes the delta based on a stored value,
/// updates the stored value, and returns the delta.
pub fn read_rotary(index: usize) -> i32 {
    if let Some((enc0, enc1)) = read_encoders() {
        let mut last_values = ENCODER_LAST_VALUES.lock().unwrap();
        let current = if index == 0 { enc0 as i32 } else { enc1 as i32 };
        let delta = current - last_values[index];
        last_values[index] = current;
        delta
    } else {
        0
    }
}
