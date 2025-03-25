use std::collections::HashMap;
use zbus::zvariant::{OwnedValue, Value};
use crate::utils::ObjectProperties;

#[derive(Debug)]
pub struct BaseGattDescriptor {
    pub path: String,
    pub uuid: String,
    pub flags: Vec<String>,
    pub characteristic: String,
}

impl BaseGattDescriptor {
    pub fn new(path: String, uuid: String, flags: Vec<String>, characteristic: String) -> Self {
        Self {
            path,
            uuid,
            flags,
            characteristic,
        }
    }

    pub fn get_properties(&self) -> ObjectProperties {
        let mut properties: ObjectProperties = HashMap::new();
        properties.insert(
            "UUID".to_string(),
            OwnedValue::try_from(Value::from(self.uuid.clone())).unwrap(),
        );
        properties.insert(
            "Flags".to_string(),
            OwnedValue::try_from(Value::from(self.flags.clone())).unwrap(),
        );
        properties.insert(
            "Characteristic".to_string(),
            OwnedValue::try_from(Value::from(self.characteristic.clone())).unwrap(),
        );

        properties
    }
}
