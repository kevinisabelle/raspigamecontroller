use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::object_path;
use crate::utils::ObjectPathTrait;
use macros::{gatt_characteristic};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::interface;

#[derive(Debug)]
pub struct BatteryLevelChrc {
    pub base: BaseGattCharacteristic,
    pub value: u8,
}

object_path! {
    impl BatteryLevelChrc {
        pub fn new(path: String, service: String) -> Self {
            let uuid = "2a19".to_string();
            let flags = vec!["read".to_string()];
            Self {
                base: BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
                value: 100,
            }
        }
    }
}

pub(crate) struct BatteryLevelChrcInterface(pub Arc<Mutex<BatteryLevelChrc>>);

#[gatt_characteristic()]
impl BatteryLevelChrcInterface {
    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<u8> {
        Ok(self.0.lock().unwrap().value)
    }
}
