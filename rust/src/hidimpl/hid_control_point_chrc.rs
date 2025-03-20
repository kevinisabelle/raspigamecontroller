use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::constants::GATT_HID_CONTROL_POINT_UUID;
use crate::object_path;
use crate::utils::ObjectPathTrait;
use macros::{gatt_characteristic};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::interface;

#[derive(Debug)]
pub struct HidControlPointChrc {
    pub base: BaseGattCharacteristic,
}

object_path! {
    impl HidControlPointChrc {
        pub fn new(path: String, service: String) -> Self {
            let uuid = GATT_HID_CONTROL_POINT_UUID.to_string();
            let flags = vec!["write".to_string()];
            Self {
                base: BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
            }
        }
    }
}

pub(crate) struct HidControlPointChrcInterface(pub Arc<Mutex<HidControlPointChrc>>);

#[gatt_characteristic()]
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
}
