use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::constants::GATT_PROTOCOL_MODE_UUID;
use crate::{extend_chrc_props, object_path};
use crate::utils::{ObjectInterfaces, ObjectPathTrait};
use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::interface;
use zbus::zvariant::{OwnedValue, Value};

#[derive(Debug)]
pub struct ProtocolModeChrc {
    pub base: BaseGattCharacteristic,
    pub value: Vec<u8>,
}

object_path! {
    impl ProtocolModeChrc {
        pub fn new(path: String, service: String) -> Self {
            let uuid = GATT_PROTOCOL_MODE_UUID.to_string();
            let flags = vec!["read".to_string(), "write".to_string()];
            Self {
                base: BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
                value: vec![0x01],
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

pub(crate) struct ProtocolModeChrcInterface(pub Arc<Mutex<ProtocolModeChrc>>);

#[gatt_characteristic()]
impl ProtocolModeChrcInterface {
    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<Vec<u8>> {
        Ok(self.0.lock().unwrap().value.clone())
    }

    fn write_value(
        &mut self,
        value: Vec<u8>,
        _options: HashMap<String, String>,
    ) -> zbus::fdo::Result<()> {
        println!(
            "Protocol Mode write handler called, Hex: {}",
            value
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ")
        );
        self.0.lock().unwrap().value = value;
        Ok(())
    }
}
