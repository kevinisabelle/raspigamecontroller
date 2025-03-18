use crate::bluez::BaseGattCharacteristic;
use crate::constants::GATT_HID_CONTROL_POINT_UUID;
use crate::utils::ObjectPathTrait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::interface;

#[derive(Debug)]
pub struct HidControlPointChrc {
    pub base: BaseGattCharacteristic,
}

impl ObjectPathTrait for HidControlPointChrc {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

impl HidControlPointChrc {
    pub fn new(path: String, service: String) -> Self {
        let uuid = GATT_HID_CONTROL_POINT_UUID.to_string();
        let flags = vec!["write".to_string()];
        Self {
            base: BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
        }
    }
}

pub(crate) struct HidControlPointChrcInterface(pub Arc<Mutex<HidControlPointChrc>>);

#[interface(name = "org.bluez.GattCharacteristic1")]
impl HidControlPointChrcInterface {
    fn write_value(
        &mut self,
        value: Vec<u8>,
        _options: HashMap<String, String>,
    ) -> zbus::fdo::Result<()> {
        println!(
            "HID Control Point write handler called, Hex: {}",
            value
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ")
        );
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
