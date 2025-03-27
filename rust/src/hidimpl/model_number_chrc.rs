use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::utils::{ObjectInterfaces, ObjectPathTrait};
use crate::{extend_chrc_props, object_path};
use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::interface;
use zbus::zvariant::{OwnedValue, Value};

#[derive(Debug)]
pub struct ModelNumberChrc {
    pub base: BaseGattCharacteristic,
    pub value: Vec<u8>,
}

object_path! {
    impl ModelNumberChrc {
        pub fn new(path: String, service: String) -> Self {
            let uuid = "2a24".to_string();
            let flags = vec!["read".to_string()];
            Self {
                base: BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
                value: "GP".as_bytes().to_vec(),
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

pub(crate) struct ModelNumberChrcInterface(pub Arc<Mutex<ModelNumberChrc>>);

#[gatt_characteristic()]
impl ModelNumberChrcInterface {
    fn read_value(&self, _options: HashMap<String, OwnedValue>) -> zbus::fdo::Result<Vec<u8>> {
        Ok(self.0.lock().unwrap().value.clone())
    }
}
