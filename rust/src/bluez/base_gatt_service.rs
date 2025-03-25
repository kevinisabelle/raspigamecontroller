use std::collections::HashMap;
use zbus::zvariant::{OwnedValue, Value};

#[derive(Debug)]
pub struct BaseGattService {
    pub path: String,
    pub uuid: String,
    pub primary: bool,
    pub characteristics: Vec<String>,
}

impl BaseGattService {
    pub fn new(path: String, uuid: String, primary: bool, characteristics: Vec<String>) -> Self {
        Self {
            path,
            uuid,
            primary,
            characteristics,
        }
    }

    pub fn get_properties(&self) -> HashMap<String, OwnedValue> {
        let mut properties = HashMap::new();
        properties.insert(
            "Primary".to_string(),
            OwnedValue::try_from(Value::from(&self.primary.clone())).unwrap(),
        );
        properties.insert(
            "UUID".to_string(),
            OwnedValue::try_from(Value::from(&self.uuid.clone())).unwrap(),
        );
        properties.insert(
            "Characteristics".to_string(),
            OwnedValue::try_from(Value::from(&self.characteristics.clone())).unwrap(),
        );

        properties
    }
}
