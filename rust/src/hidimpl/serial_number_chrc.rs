use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::constants::SERIAL_NUMBER_CHARACTERISTIC_UUID;
use crate::utils::{ObjectInterfaces, ObjectPathTrait};
use crate::{extend_chrc_props, object_path};
use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::interface;
use zbus::zvariant::{OwnedValue, Value};

#[derive(Debug)]
pub struct SerialNumberChrc {
    pub base: BaseGattCharacteristic,
    pub value: Vec<u8>,
}

object_path! {
    impl SerialNumberChrc {
        pub fn new(path: String, service: String) -> Self {
            let uuid = SERIAL_NUMBER_CHARACTERISTIC_UUID.to_string();
            let flags = vec!["read".to_string()];
            Self {
                base: BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
                value: "1.0".as_bytes().to_vec(),
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

pub(crate) struct SerialNumberChrcInterface(pub Arc<Mutex<SerialNumberChrc>>);

#[gatt_characteristic()]
impl SerialNumberChrcInterface {
    fn read_value(&self, _options: HashMap<String, OwnedValue>) -> zbus::fdo::Result<Vec<u8>> {
        Ok(self.0.lock().unwrap().value.clone())
    }
}
