use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use zbus::object_server::Interface;
use zbus::zvariant::{ObjectPath, OwnedObjectPath, OwnedValue};
use zbus::Connection;

pub trait ObjectPathTrait {
    fn object_path(&self) -> String;
}

pub async fn register_object<T>(connection: &Connection, object: Arc<T>) -> zbus::Result<()>
where
    T: Interface + Debug + ObjectPathTrait,
{
    println!("Registering object: {:?}", object);

    let path = object.object_path();
    connection
        .object_server()
        .at(path.clone(), Arc::try_unwrap(object).unwrap())
        .await?;

    println!("Registered object: {}", path);
    Ok(())
}

pub type ObjectProperties = HashMap<String, OwnedValue>;

pub type InterfaceProperties = HashMap<String, ObjectProperties>;

pub type ObjectInterfaces = HashMap<OwnedObjectPath, InterfaceProperties>;

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

#[macro_export]
macro_rules! descriptor_get_properties {
    ($self:ident) => {{
        let mut obj_props: ObjectProperties = HashMap::new();
        let base_properties = $self.base.get_properties();
        obj_props.insert(
            "Value".to_string(),
            zbus::zvariant::OwnedValue::try_from(zbus::zvariant::Value::from($self.value.clone()))
                .unwrap(),
        );
        obj_props.extend(base_properties);

        let mut interfaces: InterfaceProperties = HashMap::new();
        interfaces.insert(GATT_DESCRIPTOR_IFACE.to_string(), obj_props);

        let mut object_interfaces: ObjectInterfaces = HashMap::new();
        object_interfaces.insert(zbus::zvariant::OwnedObjectPath::from(zbus::zvariant::ObjectPath::try_from($self.object_path().clone()).unwrap()), interfaces);

        object_interfaces
    }};
}

#[macro_export]
macro_rules! extend_option_prop {
    ($desc:expr, $properties:expr) => {{
        if let Some(desc_arc) = $desc {
            if let Ok(s) = desc_arc.lock() {
                let props = s.get_properties();
                $properties.extend(props);
            }
        }
    }};
}

#[macro_export]
macro_rules! extend_chrc_props {
    ($self:expr, $props:expr, $owned_value:expr) => {{
        let base_properties = $self.base.get_properties();
        let mut service_props: ::std::collections::HashMap<String, zbus::zvariant::OwnedValue> =
            ::std::collections::HashMap::new();
        service_props.extend(base_properties);
        service_props.insert("Value".to_string(), $owned_value);
        let mut iface_props = ::std::collections::HashMap::new();
        iface_props.insert(
            $crate::constants::GATT_CHARACTERISTIC_IFACE.into(),
            service_props,
        );
        $props.insert(
            zbus::zvariant::OwnedObjectPath::from(zbus::zvariant::ObjectPath::try_from($self.object_path().clone()).unwrap()),
            iface_props,
        );
    }};
}

#[macro_export]
macro_rules! extend_service_props {
    ($self:expr, $props:expr) => {{
        let base_properties = $self.base.get_properties();
        let mut service_props: ::std::collections::HashMap<String, zbus::zvariant::OwnedValue> =
            ::std::collections::HashMap::new();
        service_props.extend(base_properties);
        let mut iface_props = ::std::collections::HashMap::new();
        iface_props.insert($crate::constants::GATT_SERVICE_IFACE.into(), service_props);
        $props.insert(
            zbus::zvariant::OwnedObjectPath::from(zbus::zvariant::ObjectPath::try_from($self.object_path().clone()).unwrap()),
            iface_props,
        );
    }};
}
