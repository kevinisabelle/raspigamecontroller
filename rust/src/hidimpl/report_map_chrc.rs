use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::constants::GATT_REPORT_MAP_UUID;
use crate::gamepad_values::GamepadValues1;
use crate::{extend_chrc_props, object_path};
use crate::utils::{ObjectInterfaces, ObjectPathTrait};
use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::interface;
use zbus::zvariant::{OwnedValue, Value};

#[derive(Debug)]
pub struct ReportMapChrc {
    pub base: BaseGattCharacteristic,
    pub gamepad_values: Arc<Mutex<GamepadValues1>>,
}

object_path! {
    impl ReportMapChrc {
        pub fn new(path: String, service: String, gamepad_values: Arc<Mutex<GamepadValues1>>) -> Self {
            let uuid = GATT_REPORT_MAP_UUID.to_string();
            let flags = vec!["read".to_string()];
            Self {
                base: BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
                gamepad_values,
            }
        }

        pub fn get_properties(&self) -> ObjectInterfaces {
            let mut properties: ObjectInterfaces = HashMap::new();
            let report_map: Vec<u16> = self.gamepad_values.lock().unwrap().get_report_map();
            let report_map_value = Value::from(report_map.iter().map(|&b| b as u8).collect::<Vec<u8>>());
            let owned_value = OwnedValue::try_from(report_map_value).unwrap();
            
            extend_chrc_props!(&self, properties, owned_value);

            properties
        }
    }
}

pub(crate) struct ReportMapChrcInterface(pub Arc<Mutex<ReportMapChrc>>);

#[gatt_characteristic()]
impl ReportMapChrcInterface {
    fn get_value(&self) -> Vec<u16> {
        let report_map = self
            .0
            .lock()
            .unwrap()
            .gamepad_values
            .lock()
            .unwrap()
            .get_report_map();
        
        println!(
            "Report Map get handler called, Hex: {}",
            report_map
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ")
        );
        
        report_map
    }

    fn read_value(&self, _options: HashMap<String, OwnedValue>) -> zbus::fdo::Result<Vec<u16>> {
        let report_map = self
            .0
            .lock()
            .unwrap()
            .gamepad_values
            .lock()
            .unwrap()
            .get_report_map();
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
