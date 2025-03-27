use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::constants::GATT_HID_INFORMATION_UUID;
use crate::utils::{ObjectInterfaces, ObjectPathTrait};
use crate::{extend_chrc_props, object_path};
use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::interface;
use zbus::zvariant::{OwnedValue, Value};

#[derive(Debug)]
pub struct HidInfoChrc {
    pub base: BaseGattCharacteristic,
    pub value: Vec<u8>,
}

object_path! {
    impl HidInfoChrc {
        pub fn new(path: String, service: String) -> Self {
            let uuid = GATT_HID_INFORMATION_UUID.to_string();
            let flags = vec!["read".to_string()];
            Self {
                base: BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
                value: vec![0x11, 0x01, 0x00, 0x03], // bcdHID, bCountryCode, Flags (RemoteWake, NormallyConnectable)
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

pub(crate) struct HidInfoChrcInterface(pub Arc<Mutex<HidInfoChrc>>);

#[gatt_characteristic()]
impl HidInfoChrcInterface {
    fn read_value(&self, _options: HashMap<String, OwnedValue>) -> zbus::fdo::Result<Vec<u8>> {
        let value = self.0.lock().unwrap().value.clone();
        println!(
            "HID Information read handler called, Hex: {}",
            value
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ")
        );
        Ok(value)
    }
}
