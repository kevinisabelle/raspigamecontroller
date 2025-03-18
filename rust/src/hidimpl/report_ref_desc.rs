use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::interface;
use crate::bluez::BaseGattDescriptor;
use crate::constants::GATT_DESC_CLIENT_DESCRIPTOR_UUID;
use crate::utils::ObjectPathTrait;

#[derive(Debug)]
pub struct ReportReferenceDesc {
    pub base: BaseGattDescriptor,
    pub value: Vec<u8>,
}

impl ObjectPathTrait for ReportReferenceDesc {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

impl ReportReferenceDesc {
    pub fn new(path: String, characteristic: String) -> Self {
        Self {
            base: BaseGattDescriptor::new(
                path,
                GATT_DESC_CLIENT_DESCRIPTOR_UUID.to_string(),
                vec!["read".to_string()],
                characteristic,
            ),
            value: vec![0x00, 0x01],
        }
    }

    pub fn set_value(&mut self, value: Vec<u8>) {
        self.value = value;
    }
}

pub(crate) struct ReportReferenceDescInterface(pub Arc<Mutex<ReportReferenceDesc>>);

#[interface(name = "org.bluez.GattDescriptor1")]
impl ReportReferenceDescInterface {
    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<Vec<u8>> {
        Ok(self.0.lock().unwrap().value.clone())
    }

    #[zbus(property)]
    fn get_flags(&self) -> Vec<String> {
        self.0.lock().unwrap().base.flags.clone()
    }

    #[zbus(property)]
    fn get_uuid(&self) -> String {
        self.0.lock().unwrap().base.uuid.clone()
    }

    #[zbus(property)]
    fn get_characteristic(&self) -> String {
        self.0.lock().unwrap().base.characteristic.clone()
    }
}