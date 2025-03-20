use crate::constants::GATT_HID_INFORMATION_UUID;
use crate::utils::ObjectPathTrait;
use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::interface;
use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;

#[derive(Debug)]
pub struct HidInfoChrc {
    pub base: BaseGattCharacteristic,
    pub value: Vec<u8>,
}

impl ObjectPathTrait for HidInfoChrc {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

impl HidInfoChrc {
    pub fn new(path: String, service: String) -> Self {
        let uuid = GATT_HID_INFORMATION_UUID.to_string();
        let flags = vec!["read".to_string()];
        Self {
            base: BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
            value: vec![0x11, 0x01, 0x00, 0x03], // bcdHID, bCountryCode, Flags (RemoteWake, NormallyConnectable)
        }
    }
}

pub(crate) struct HidInfoChrcInterface(pub Arc<Mutex<HidInfoChrc>>);

#[gatt_characteristic()]
impl HidInfoChrcInterface {
    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<Vec<u8>> {
        Ok(self.0.lock().unwrap().value.clone())
    }
}
