use std::fmt::Debug;
use std::sync::Arc;
use zbus::Connection;
use zbus::object_server::Interface;

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
        Err(e) => {
            println!("Failed to unwrap object: {:?}", &e.object_path());
            return Err(zbus::Error::NameTaken);
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

