use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::gatt_chrc_properties;
use crate::utils::ObjectPathTrait;
use paste::paste;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::interface;

#[derive(Debug)]
pub struct BatteryLevelChrc {
    pub base: BaseGattCharacteristic,
    pub value: u8,
}

impl ObjectPathTrait for BatteryLevelChrc {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

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

pub(crate) struct BatteryLevelChrcInterface(pub Arc<Mutex<BatteryLevelChrc>>);

#[interface(name = "org.bluez.GattCharacteristic1")]
impl BatteryLevelChrcInterface {
    gatt_chrc_properties!();

    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<u8> {
        Ok(self.0.lock().unwrap().value)
    }
}
