use crate::bluez::BaseGattCharacteristic;
use crate::constants::GATT_HID_INFORMATION_UUID;
use crate::utils::ObjectPathTrait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::interface;

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

#[interface(name = "org.bluez.GattCharacteristic1")]
impl HidInfoChrcInterface {
    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<Vec<u8>> {
        Ok(self.value.clone())
    }

    #[zbus(property)]
    fn get_flags(&self) -> Vec<String> {
        self.base.flags.clone()
    }

    #[zbus(property)]
    fn get_uuid(&self) -> String {
        self.base.uuid.clone()
    }

    #[zbus(property)]
    fn get_service(&self) -> String {
        self.base.service.clone()
    }

    #[zbus(property)]
    fn get_descriptors(&self) -> Vec<String> {
        self.base.descriptors.clone()
    }
}
