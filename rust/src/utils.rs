use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use zbus::object_server::Interface;
use zbus::Connection;

pub trait ObjectPathTrait {
    fn object_path(&self) -> String;
}

pub async fn register_object<T>(connection: &Connection, object: Arc<T>) -> zbus::Result<()>
where
    T: ObjectPathTrait + Interface + Debug,
{
    println!("Registering object: {:?}", object);
    let object_unwrapped = match Arc::try_unwrap(object) {
        Ok(o) => o,
        Err(_) => {
            panic!("Failed to unwrap object");
        }
    };
    let object_path = object_unwrapped.object_path().clone();
    connection
        .object_server()
        .at(object_path.to_string(), object_unwrapped)
        .await?;

    println!("Registered object: {}", object_path);
    Ok(())
}

pub type Properties<'a> = HashMap<String, zbus::zvariant::Value<'a>>;
