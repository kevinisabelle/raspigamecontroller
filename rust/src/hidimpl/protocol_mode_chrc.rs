use crate::bluez::BaseGattCharacteristic;
use crate::constants::GATT_PROTOCOL_MODE_UUID;
use crate::utils::ObjectPathTrait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::interface;

#[derive(Debug)]
pub struct ProtocolModeChrc {
    pub base: BaseGattCharacteristic,
    pub value: Vec<u8>,
}

impl ObjectPathTrait for ProtocolModeChrc {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

impl ProtocolModeChrc {
    pub fn new(path: String, service: String) -> Self {
        let uuid = GATT_PROTOCOL_MODE_UUID.to_string();
        let flags = vec!["read".to_string(), "write".to_string()];
        Self {
            base: BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
            value: vec![0x01],
        }
    }
}

pub(crate) struct ProtocolModeChrcInterface(pub Arc<Mutex<ProtocolModeChrc>>);

#[interface(name = "org.bluez.GattCharacteristic1")]
impl ProtocolModeChrcInterface {
    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<Vec<u8>> {
        Ok(self.value.clone())
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
        self.value = value;
        Ok(())
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
