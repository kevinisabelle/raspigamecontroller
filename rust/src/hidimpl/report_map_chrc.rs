use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::constants::GATT_REPORT_MAP_UUID;
use crate::gamepad_values::GamepadValues1;
use crate::utils::ObjectPathTrait;
use macros::gatt_chrc_properties;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::interface;
use zbus::zvariant::Value;

#[derive(Debug)]
pub struct ReportMapChrc {
    pub base: BaseGattCharacteristic,
    pub gamepad_values: Arc<GamepadValues1>,
}

impl ObjectPathTrait for ReportMapChrc {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

impl ReportMapChrc {
    pub fn new(path: String, service: String, gamepad_values: Arc<GamepadValues1>) -> Self {
        let uuid = GATT_REPORT_MAP_UUID.to_string();
        let flags = vec!["read".to_string()];
        Self {
            base: BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
            gamepad_values,
        }
    }

    pub fn get_properties(&self) -> HashMap<String, Value> {
        let mut properties: HashMap<String, Value> = HashMap::new();
        properties.insert(
            "Value".to_string(),
            Value::from(self.gamepad_values.get_report_map()),
        );

        let base_properties = self.base.get_properties();

        for (key, value) in base_properties {
            properties.insert(key, value);
        }

        properties
    }
}

pub(crate) struct ReportMapChrcInterface(pub Arc<Mutex<ReportMapChrc>>);

#[interface(name = "org.bluez.GattCharacteristic1")]
#[gatt_chrc_properties()]
impl ReportMapChrcInterface {

    fn get_value(&self) -> Vec<u8> {
        let report_map = self.0.lock().unwrap().gamepad_values.get_report_map();
        report_map
    }

    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<Vec<u8>> {
        let report_map = self.0.lock().unwrap().gamepad_values.get_report_map();
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
