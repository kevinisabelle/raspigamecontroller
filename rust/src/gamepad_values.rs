﻿pub struct GamepadValues1;
impl GamepadValues1 {
    pub fn get_report_map(&self) -> Vec<u8> {
        // Return a sample report map (in a real use-case, return actual map bytes).
        vec![0x01, 0x02, 0x03, 0x04]
    }
}