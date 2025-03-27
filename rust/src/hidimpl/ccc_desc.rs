use crate::bluez::base_gatt_desc::BaseGattDescriptor;
use crate::constants::{GATT_DESCRIPTOR_IFACE, GATT_DESC_CLIENT_DESCRIPTOR_UUID};
use crate::utils::{InterfaceProperties, ObjectInterfaces, ObjectPathTrait, ObjectProperties};
use crate::{descriptor_get_properties, object_path};
use macros::gatt_descriptor;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use zbus::interface;
use zbus::zvariant::OwnedValue;

#[derive(Debug)]
pub struct ClientCharacteristicConfigurationDesc {
    pub base: BaseGattDescriptor,
    pub value: Vec<u8>,
}

object_path! {
    impl ClientCharacteristicConfigurationDesc {
        pub fn new(path: String, characteristic: String) -> Self {
            Self {
                base: BaseGattDescriptor::new(
                    path,
                    GATT_DESC_CLIENT_DESCRIPTOR_UUID.to_string(),
                    vec!["read".to_string(), "write".to_string()],
                    characteristic,
                ),
                value: vec![0x00, 0x00],
            }
        }

        pub fn get_properties(&self) -> ObjectInterfaces {
            descriptor_get_properties!(self)
        }
    }
}

pub(crate) struct CCCDescInterface(pub Arc<Mutex<ClientCharacteristicConfigurationDesc>>);

#[gatt_descriptor()]
impl CCCDescInterface {
    fn read_value(&self, _options: HashMap<String, OwnedValue>) -> zbus::fdo::Result<Vec<u8>> {
        let value = self.0.lock().unwrap().value.clone();
        println!(
            "Client Characteristic Configuration Descriptor read handler called, Hex: {}",
            value
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ")
        );
        Ok(value)
    }

    fn write_value(
        &mut self,
        value: Vec<u8>,
        _options: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<()> {
        println!(
            "Client Characteristic Configuration Descriptor write handler called, Hex: {}",
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
