use std::collections::HashMap;
use std::fmt::Debug;
use zbus::object_server::Interface;
use zbus::Connection;

pub trait ObjectPathTrait {
    fn object_path(&self) -> String;
}

pub async fn register_object<T>(
    connection: &Connection,
    object: T,
    path: String,
) -> zbus::Result<()>
where
    T: Interface + Debug,
{
    println!("Registering object: {:?}", object);

    connection.object_server().at(path.clone(), object).await?;

    println!("Registered object: {}", path);
    Ok(())
}

pub type Properties<'a> = HashMap<String, zbus::zvariant::Value<'a>>;
