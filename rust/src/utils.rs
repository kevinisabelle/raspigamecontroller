use std::collections::HashMap;

pub fn insert_if_some<'a, T>(map: &mut HashMap<String, zbus::zvariant::Value<'a>>, key: &str, option: &Option<T>)
where
    T: Clone,
    zbus::zvariant::Value<'a>: From<T>,
{
    if let Some(v) = option {
        map.insert(key.to_string(), zbus::zvariant::Value::from(v.clone()));
    }
}