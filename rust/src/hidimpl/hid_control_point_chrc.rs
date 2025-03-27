use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::constants::GATT_HID_CONTROL_POINT_UUID;
use crate::utils::{ObjectInterfaces, ObjectPathTrait};
use crate::{extend_chrc_props, object_path};
use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::interface;
use zbus::zvariant::{OwnedValue, Value};

#[derive(Debug)]
pub struct HidControlPointChrc {
    pub base: BaseGattCharacteristic,
    pub value: Vec<u8>,
}

object_path! {
    impl HidControlPointChrc {
        pub fn new(path: String, service: String) -> Self {
            let uuid = GATT_HID_CONTROL_POINT_UUID.to_string();
            let flags = vec!["write".to_string()];
            Self {
                base: BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
                value: vec![0x00],
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

pub(crate) struct HidControlPointChrcInterface(pub Arc<Mutex<HidControlPointChrc>>);

#[gatt_characteristic()]
impl HidControlPointChrcInterface {
    fn write_value(
        &mut self,
        value: Vec<u8>,
        _options: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<()> {
        println!(
            "HID Control Point write handler called, Hex: {}",
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
