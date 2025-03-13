// rust
use crate::bluez::{Advertisement, BasicCharacteristic, GattCharacteristic};
use crate::gamepad_values::GamepadValues1;
use std::collections::HashMap;
use zbus::interface;

// For the gamepad appearance constant, define a constant value. Adjust as needed.
const ADV_APPEARANCE_GAMEPAD: u16 = 0x03c0;

pub fn create_advertisement(path: &str) -> Advertisement {
    let mut adv = Advertisement::new(path, "peripheral".to_string());
    adv.add_service_uuid("1812".to_string());
    adv.add_local_name("KiGP".to_string());
    // Set the appearance and include_tx_power fields
    adv.set_appearance(ADV_APPEARANCE_GAMEPAD);
    adv.set_include_tx_power(true);
    adv
}

pub struct ReportMapChrc {
    pub base: BasicCharacteristic,
    pub gamepad_values: GamepadValues1,
}

impl ReportMapChrc {
    pub fn new(path: String, service: String, gamepad_values: GamepadValues1) -> Self {
        // Here we use a constant UUID and flag for a report map.
        let uuid = "GATT_REPORT_MAP_UUID".to_string();
        let flags = vec!["read".to_string()];
        Self {
            base: BasicCharacteristic::new(path, uuid, flags, service),
            gamepad_values,
        }
    }
}

#[interface(name = "org.bluez.GattCharacteristic1")]
impl GattCharacteristic for ReportMapChrc {
    // Override read_value to return the report map.
    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<Vec<u8>> {
        let report_map = self.gamepad_values.get_report_map();
        println!(
            "Report Map read handler called, Hex: {}",
            report_map
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ")
        );
        Ok(report_map)
    }
}
