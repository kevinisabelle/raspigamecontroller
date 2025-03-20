#[derive(Debug)]
pub struct GamepadValues1;

impl GamepadValues1 {
    pub(crate) fn new() -> Self {
        Self
    }
    pub fn get_report_map(&self) -> Vec<u8> {
        // Return a sample report map (in a real use-case, return actual map bytes).
        vec![0x01, 0x02, 0x03, 0x04]
    }

    pub fn get_report(&self) -> Vec<u8> {
        // Return a sample report (in a real use-case, return actual report bytes).
        vec![0x01, 0x02, 0x03, 0x04]
    }
}
