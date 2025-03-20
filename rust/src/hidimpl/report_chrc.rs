use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::constants::GATT_REPORT_UUID;
use crate::gamepad_values::GamepadValues1;
use crate::utils::ObjectPathTrait;
use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::interface;

#[derive(Debug)]
pub struct ReportChrc {
    pub base: BaseGattCharacteristic,
    pub gamepad_values: Arc<GamepadValues1>,
}

impl ObjectPathTrait for ReportChrc {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

impl ReportChrc {
    pub fn new(path: String, service: String, gamepad_values: Arc<GamepadValues1>) -> Self {
        let uuid = GATT_REPORT_UUID.to_string();
        let flags = vec!["read".to_string(), "write".to_string()];

        Self {
            base: BaseGattCharacteristic::new(path.clone(), uuid, flags, service, vec![]),
            gamepad_values,
        }
    }

    pub fn add_descriptor_path(&mut self, path: String) {
        self.base.descriptors.push(path);
    }
}

pub(crate) struct ReportChrcInterface(pub Arc<Mutex<ReportChrc>>);

#[gatt_characteristic()]
impl ReportChrcInterface {
    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<Vec<u8>> {
        let gamepad_values = self.0.lock().unwrap().gamepad_values.clone();
        let report = gamepad_values.get_report_map();
        println!(
            "Report read handler called, Hex: {}",
            report
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ")
        );
        Ok(report)
    }

    fn write_value(
        &mut self,
        value: Vec<u8>,
        _options: HashMap<String, String>,
    ) -> zbus::fdo::Result<()> {
        println!(
            "Report write handler called, Hex: {}",
            value
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ")
        );
        Ok(())
    }
}
