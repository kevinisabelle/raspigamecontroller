use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::constants::GATT_REPORT_UUID;
use crate::gamepad_values::GamepadValues1;
use crate::hidimpl::ccc_desc::ClientCharacteristicConfigurationDesc;
use crate::hidimpl::report_ref_desc::ReportReferenceDesc;
use crate::utils::{ObjectInterfaces, ObjectPathTrait};
use crate::{extend_chrc_props, extend_option_prop, object_path};
use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::names::{InterfaceName, MemberName};
use zbus::zvariant::{OwnedValue, Value};
use zbus::{interface, Connection};

#[derive(Debug)]
pub struct ReportChrc {
    pub base: BaseGattCharacteristic,
    pub gamepad_values: Arc<Mutex<GamepadValues1>>,
    pub rr_desc: Option<Arc<Mutex<ReportReferenceDesc>>>,
    pub ccc_desc: Option<Arc<Mutex<ClientCharacteristicConfigurationDesc>>>,
    pub notifying: bool,
    pub connection: Connection, // Store the connection
}

object_path! {
    impl ReportChrc {
        pub fn new(path: String, service: String, gamepad_values: Arc<Mutex<GamepadValues1>>, connection: Connection) -> Self {
            let uuid = GATT_REPORT_UUID.to_string();
            let flags = vec!["read".to_string(), "notify".to_string(), "write-without-response".to_string()];

            Self {
                base: BaseGattCharacteristic::new(path.clone(), uuid, flags, service, vec![]),
                gamepad_values,
                notifying: false,
                rr_desc: None,
                ccc_desc: None,
                connection
            }
        }

        pub fn add_descriptor_path(&mut self, path: String) {
            self.base.descriptors.push(path);
        }

        pub async fn emit_report_changed_signal(&self) -> zbus::Result<()> {

            if !self.notifying {
                return Ok(());
            }

            let object_path = self.object_path();
            let gamepad_values = self.gamepad_values.clone();
            let connection = self.connection.clone();

            let report = gamepad_values.lock().unwrap().get_report().clone();

            let mut changed: HashMap<String, OwnedValue> = HashMap::new();
            let report_value = OwnedValue::try_from(Value::from(report.clone())).unwrap();
            changed.insert("Value".to_string(), report_value);

            let interface_name = InterfaceName::try_from("org.bluez.GattCharacteristic1".to_string()).unwrap();
            let signal_name = MemberName::try_from("PropertiesChanged".to_string()).unwrap();
            let interface_name_props = InterfaceName::try_from("org.freedesktop.DBus.Properties".to_string()).unwrap();

            connection.emit_signal(
                    None::<&str>,
                    object_path.as_str(),
                    interface_name_props.as_str(),
                    signal_name.as_str(),
                    &(
                        interface_name.as_str(),
                        changed,
                        Vec::<String>::new(),
                    ),
                )
                .await?;

            /*println!(
                "Report changed signal emitted, Hex: {}",
                report
                    .iter()
                    .map(|b| format!("{:02X}", b))
                    .collect::<Vec<_>>()
                    .join(" ")
            );*/

            Ok(())
        }

        pub fn get_properties(&self) -> ObjectInterfaces {
            let mut properties = HashMap::new();
            let owned_value = OwnedValue::try_from(Value::from(self.gamepad_values.lock().unwrap().get_report().clone())).unwrap();
            extend_chrc_props!(&self, properties, owned_value);
            extend_option_prop!(&self.ccc_desc, properties);
            extend_option_prop!(&self.rr_desc, properties);

            properties
        }
    }
}

pub(crate) struct ReportChrcInterface(pub Arc<Mutex<ReportChrc>>);

#[gatt_characteristic()]
impl ReportChrcInterface {
    fn read_value(&self, _options: HashMap<String, OwnedValue>) -> zbus::fdo::Result<Vec<u8>> {
        let gamepad_values = self.0.lock().unwrap().gamepad_values.clone();
        let report = gamepad_values.lock().unwrap().get_report().clone();
        println!(
            "Report read handler called, Hex: {}",
            report
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ")
        );
        Ok(report)
    }

    fn write_value(
        &mut self,
        value: Vec<u8>,
        _options: HashMap<String, OwnedValue>,
    ) -> zbus::fdo::Result<()> {
        println!(
            "Report write handler called, Hex: {}",
            value
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ")
        );
        Ok(())
    }

    fn start_notify(&mut self) -> zbus::fdo::Result<()> {
        self.0.lock().unwrap().notifying = true;
        println!("Report start notify called");
        Ok(())
    }

    fn stop_notify(&mut self) -> zbus::fdo::Result<()> {
        self.0.lock().unwrap().notifying = false;
        println!("Report stop notify called");
        Ok(())
    }
}
