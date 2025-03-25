use crate::bluez::base_gatt_desc::BaseGattDescriptor;
use crate::constants::{GATT_DESCRIPTOR_IFACE, GATT_DESC_CLIENT_DESCRIPTOR_UUID};
use crate::{descriptor_get_properties, object_path};
use crate::utils::{InterfaceProperties, ObjectInterfaces, ObjectPathTrait, ObjectProperties};
use macros::gatt_descriptor;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::interface;

#[derive(Debug)]
pub struct ReportReferenceDesc {
    pub base: BaseGattDescriptor,
    pub value: Vec<u8>,
}

object_path! {
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
        
        pub fn get_properties(&self) -> ObjectInterfaces {
            descriptor_get_properties!(self)
        }
    }
}

pub(crate) struct ReportReferenceDescInterface(pub Arc<Mutex<ReportReferenceDesc>>);

#[gatt_descriptor()]
impl ReportReferenceDescInterface {
    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<Vec<u8>> {
        Ok(self.0.lock().unwrap().value.clone())
    }
}
