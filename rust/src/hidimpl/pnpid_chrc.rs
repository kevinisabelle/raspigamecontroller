use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::utils::{ObjectInterfaces, ObjectPathTrait};
use crate::{extend_chrc_props, object_path};
use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::interface;
use zbus::zvariant::{OwnedValue, Value};

#[derive(Debug)]
pub struct PnpIdChrc {
    pub base: BaseGattCharacteristic,
    pub value: Vec<u8>,
}

object_path! {
    impl PnpIdChrc {
        pub fn new(path: String, service: String) -> Self {
            let uuid = "2a50".to_string();
            let flags = vec!["read".to_string()];
            Self {
                base: BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
                value: vec![
                    0x0, 0x2, 0x6, 0xD, 0x0, 0x4, 0x7, 0x8, 0x5, 0x6, 0x0, 0x0, 0x0, 0x1,
                ],
            }
        }

        pub fn get_properties(&self) -> ObjectInterfaces {

            let mut properties = HashMap::new();
            let owned_value = OwnedValue::try_from(Value::from(self.value.clone())).unwrap();

            extend_chrc_props!(&self, properties, owned_value);

            properties
        }
    }
}

pub(crate) struct PnpIdChrcInterface(pub Arc<Mutex<PnpIdChrc>>);

#[gatt_characteristic()]
impl PnpIdChrcInterface {
    fn read_value(&self, _options: HashMap<String, OwnedValue>) -> zbus::fdo::Result<Vec<u8>> {
        Ok(self.0.lock().unwrap().value.clone())
    }
}
