#[derive(Debug)]
pub struct GamepadValues1 {
    pub btn10: bool,
    pub btn11: bool,
    pub btn12: bool,
    pub btn13: bool,
    pub btn14: bool,
    pub btn15: bool,
    pub btn16: bool,
    pub btn17: bool,
    pub slider0: u8,
    pub axis_x0: u8,
    pub axis_y0: u8,
    pub axis_z0: u8,
    pub axis_rx0: u8,
    pub axis_ry0: u8,
    pub axis_rz0: u8,
    pub axis_vx0: u8,
}

impl GamepadValues1 {
    pub(crate) fn new() -> Self {
        Self {
            btn10: false,
            btn11: false,
            btn12: false,
            btn13: false,
            btn14: false,
            btn15: false,
            btn16: false,
            btn17: false,
            slider0: 0,
            axis_x0: 0,
            axis_y0: 0,
            axis_z0: 0,
            axis_rx0: 0,
            axis_ry0: 0,
            axis_rz0: 0,
            axis_vx0: 0,
        }
    }

    pub fn set_btn10(&mut self, value: bool) {
        self.btn10 = value;
    }

    pub fn set_btn11(&mut self, value: bool) {
        self.btn11 = value;
    }

    pub fn set_btn12(&mut self, value: bool) {
        self.btn12 = value;
    }

    pub fn set_btn13(&mut self, value: bool) {
        self.btn13 = value;
    }

    pub fn set_btn14(&mut self, value: bool) {
        self.btn14 = value;
    }

    pub fn set_btn15(&mut self, value: bool) {
        self.btn15 = value;
    }

    pub fn set_btn16(&mut self, value: bool) {
        self.btn16 = value;
    }

    pub fn set_btn17(&mut self, value: bool) {
        self.btn17 = value;
    }

    pub fn set_slider0(&mut self, value: u8) {
        self.slider0 = value & 0xFF;
    }

    pub fn set_axis_x0(&mut self, value: u8) {
        self.axis_x0 = value & 0xFF;
    }

    pub fn set_axis_y0(&mut self, value: u8) {
        self.axis_y0 = value & 0xFF;
    }

    pub fn set_axis_z0(&mut self, value: u8) {
        self.axis_z0 = value & 0xFF;
    }

    pub fn set_axis_rx0(&mut self, value: u8) {
        self.axis_rx0 = value & 0xFF;
    }

    pub fn set_axis_ry0(&mut self, value: u8) {
        self.axis_ry0 = value & 0xFF;
    }

    pub fn set_axis_rz0(&mut self, value: u8) {
        self.axis_rz0 = value & 0xFF;
    }

    pub fn set_axis_vx0(&mut self, value: u8) {
        self.axis_vx0 = value & 0xFF;
    }

    pub fn get_report_map(&self) -> Vec<u16> {
        // Return a sample report map (in a real use-case, return actual map bytes).
        vec![
            0x05, 0x01, // Usage Page (Generic Desktop Controls)
            0x09, 0x05, // Usage (Gamepad)
            0xA1, 0x01, // Collection (Application)
            0x85, 0x01, // Report ID 1
            0x05, 0x09, // Usage Page (Button)
            0x19, 0x01, // Usage Minimum (1)
            0x29, 0x08, // Usage Maximum (8)
            0x15, 0x00, // Logical Minimum (0)
            0x25, 0x01, // Logical Maximum (1)
            0x75, 0x01, // Report Size (1)
            0x95, 0x08, // Report Count (8)
            0x81, 0x02, // Input (00000010)
            0x05, 0x01, // Usage Page (Generic Desktop Controls)
            0x09, 0x36, // Usage (Slider)
            0x19, 0x01, // Usage Minimum (1)
            0x29, 0x01, // Usage Maximum (1)
            0x15, 0x00, // Logical Minimum (0)
            0x25, 0xFF, // Logical Maximum (255)
            0x35, 0x00, // Physical Minimum (0)
            0x45, 0xFF, // Physical Maximum (255)
            0x75, 0x08, // Report Size (8)
            0x95, 0x01, // Report Count (1)
            0x81, 0x02, // Input (00000010)
            0x05, 0x01, // Usage Page (Generic Desktop Controls)
            0x09, 0x30, // Usage (X)
            0x19, 0x01, // Usage Minimum (1)
            0x29, 0x01, // Usage Maximum (1)
            0x15, 0x81, // Logical Minimum (-127)
            0x25, 0x7F, // Logical Maximum (127)
            0x35, 0x81, // Physical Minimum (-127)
            0x45, 0x7F, // Physical Maximum (127)
            0x75, 0x08, // Report Size (8)
            0x95, 0x01, // Report Count (1)
            0x81, 0x02, // Input (00000010)
            0x05, 0x01, // Usage Page (Generic Desktop Controls)
            0x09, 0x31, // Usage (Y)
            0x19, 0x01, // Usage Minimum (1)
            0x29, 0x01, // Usage Maximum (1)
            0x15, 0x00, // Logical Minimum (0)
            0x25, 0xFF, // Logical Maximum (255)
            0x35, 0x00, // Physical Minimum (0)
            0x45, 0xFF, // Physical Maximum (255)
            0x75, 0x08, // Report Size (8)
            0x95, 0x01, // Report Count (1)
            0x81, 0x02, // Input (00000010)
            0x05, 0x01, // Usage Page (Generic Desktop Controls)
            0x09, 0x32, // Usage (Z)
            0x19, 0x01, // Usage Minimum (1)
            0x29, 0x01, // Usage Maximum (1)
            0x15, 0x00, // Logical Minimum (0)
            0x25, 0xFF, // Logical Maximum (255)
            0x35, 0x00, // Physical Minimum (0)
            0x45, 0xFF, // Physical Maximum (255)
            0x75, 0x08, // Report Size (8)
            0x95, 0x01, // Report Count (1)
            0x81, 0x02, // Input (00000010)
            0x05, 0x01, // Usage Page (Generic Desktop Controls)
            0x09, 0x33, // Usage (Rx)
            0x19, 0x01, // Usage Minimum (1)
            0x29, 0x01, // Usage Maximum (1)
            0x15, 0x00, // Logical Minimum (0)
            0x25, 0xFF, // Logical Maximum (255)
            0x35, 0x00, // Physical Minimum (0)
            0x45, 0xFF, // Physical Maximum (255)
            0x75, 0x08, // Report Size (8)
            0x95, 0x01, // Report Count (1)
            0x81, 0x02, // Input (00000010)
            0x05, 0x01, // Usage Page (Generic Desktop Controls)
            0x09, 0x34, // Usage (Ry)
            0x19, 0x01, // Usage Minimum (1)
            0x29, 0x01, // Usage Maximum (1)
            0x15, 0x00, // Logical Minimum (0)
            0x25, 0xFF, // Logical Maximum (255)
            0x35, 0x00, // Physical Minimum (0)
            0x45, 0xFF, // Physical Maximum (255)
            0x75, 0x08, // Report Size (8)
            0x95, 0x01, // Report Count (1)
            0x81, 0x02, // Input (00000010)
            0x05, 0x01, // Usage Page (Generic Desktop Controls)
            0x09, 0x35, // Usage (Rz)
            0x19, 0x01, // Usage Minimum (1)
            0x29, 0x01, // Usage Maximum (1)
            0x15, 0x00, // Logical Minimum (0)
            0x25, 0xFF, // Logical Maximum (255)
            0x35, 0x00, // Physical Minimum (0)
            0x45, 0xFF, // Physical Maximum (255)
            0x75, 0x08, // Report Size (8)
            0x95, 0x01, // Report Count (1)
            0x81, 0x02, // Input (00000010)
            0x05, 0x01, // Usage Page (Generic Desktop Controls)
            0x09, 0x30, // Usage (X)
            0x19, 0x01, // Usage Minimum (1)
            0x29, 0x01, // Usage Maximum (1)
            0x15, 0x00, // Logical Minimum (0)
            0x25, 0xFF, // Logical Maximum (255)
            0x35, 0x00, // Physical Minimum (0)
            0x45, 0xFF, // Physical Maximum (255)
            0x75, 0x08, // Report Size (8)
            0x95, 0x01, // Report Count (1)
            0x81, 0x02, // Input (00000010)
            0xC0, // End Collection
        ]
    }

    // Assembles a report by concatenating the 8 one-bit button values and eight 8-bit fields.
    /// A report ID (0x01) is prepended to the resulting 9-byte value (total 10 bytes).
    pub fn get_report(&self) -> Vec<u8> {
        // Use a u128 to accumulate 72 bits
        let mut total: u128 = 0;
        total = (total << 1) | self.btn10 as u128;
        total = (total << 1) | self.btn11 as u128;
        total = (total << 1) | self.btn12 as u128;
        total = (total << 1) | self.btn13 as u128;
        total = (total << 1) | self.btn14 as u128;
        total = (total << 1) | self.btn15 as u128;
        total = (total << 1) | self.btn16 as u128;
        total = (total << 1) | self.btn17 as u128;
        total = (total << 8) | self.slider0 as u128;
        total = (total << 8) | self.axis_x0 as u128;
        total = (total << 8) | self.axis_y0 as u128;
        total = (total << 8) | self.axis_z0 as u128;
        total = (total << 8) | self.axis_rx0 as u128;
        total = (total << 8) | self.axis_ry0 as u128;
        total = (total << 8) | self.axis_rz0 as u128;
        total = (total << 8) | self.axis_vx0 as u128;

        // Convert the 72-bit value into 9 bytes. total.to_be_bytes() gives 16 bytes.
        let all_bytes = total.to_be_bytes();
        let report_bytes = all_bytes[16 - 9..].to_vec();

        // Prepend report ID 0x01
        let mut result = Vec::with_capacity(10);
        result.push(0x01);
        result.extend(report_bytes);
        result
    }
}
