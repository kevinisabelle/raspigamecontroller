use crate::bluez::base_gatt_desc::BaseGattDescriptor;
use crate::constants::{GATT_DESC_CLIENT_DESCRIPTOR_UUID, GATT_DESCRIPTOR_IFACE, GATT_DESC_REPORT_REFERENCE_UUID};
use crate::utils::{InterfaceProperties, ObjectInterfaces, ObjectPathTrait, ObjectProperties};
use crate::{descriptor_get_properties, object_path};
use macros::gatt_descriptor;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::interface;
use zbus::zvariant::OwnedValue;

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
                    GATT_DESC_REPORT_REFERENCE_UUID.to_string(),
                    vec!["read".to_string()],
                    characteristic,
                ),
                value: vec![0x00, 0x01],
            }
        }

        pub fn get_properties(&self) -> ObjectInterfaces {
            descriptor_get_properties!(self)
        }
    }
}

pub(crate) struct ReportReferenceDescInterface(pub Arc<Mutex<ReportReferenceDesc>>);

#[gatt_descriptor()]
impl ReportReferenceDescInterface {
    fn read_value(&self, _options: HashMap<String, OwnedValue>) -> zbus::fdo::Result<Vec<u8>> {
        Ok(self.0.lock().unwrap().value.clone())
    }
}
