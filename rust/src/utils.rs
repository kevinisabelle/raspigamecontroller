use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use zbus::object_server::Interface;
use zbus::Connection;

pub trait ObjectPathTrait {
    fn object_path(&self) -> String;
}

pub async fn register_object<T>(
    connection: &Connection,
    object: Arc<T>,
) -> zbus::Result<()>
where
    T: Interface + Debug + ObjectPathTrait,
{
    println!("Registering object: {:?}", object);

    let path = object.object_path();
    connection.object_server().at(path.clone(), Arc::try_unwrap(object).unwrap()).await?;

    println!("Registered object: {}", path);
    Ok(())
}

pub type Properties<'a> = HashMap<String, zbus::zvariant::Value<'a>>;

#[macro_export]
macro_rules! object_path {
    (impl $ty:ty { $($body:tt)* }) => {
        impl $ty {
            $($body)*
        }
        impl ObjectPathTrait for $ty {
            fn object_path(&self) -> String {
                self.base.path.to_string()
            }
        }
    };
}