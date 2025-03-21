use crate::bluez::base_gatt_chrc::BaseGattCharacteristic;
use crate::constants::GATT_REPORT_UUID;
use crate::gamepad_values::GamepadValues1;
use crate::object_path;
use crate::utils::ObjectPathTrait;
use macros::gatt_characteristic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::interface;
use zbus::object_server::SignalEmitter;

#[derive(Debug)]
pub struct ReportChrc {
    pub base: BaseGattCharacteristic,
    pub gamepad_values: Arc<Mutex<GamepadValues1>>,
    pub notifying: bool,
}

object_path! {
    impl ReportChrc {
        pub fn new(path: String, service: String, gamepad_values: Arc<Mutex<GamepadValues1>>) -> Self {
            let uuid = GATT_REPORT_UUID.to_string();
            let flags = vec!["read".to_string(), "write".to_string()];

            Self {
                base: BaseGattCharacteristic::new(path.clone(), uuid, flags, service, vec![]),
                gamepad_values,
                notifying: false,
            }
        }

        pub fn add_descriptor_path(&mut self, path: String) {
            self.base.descriptors.push(path);
        }

        pub fn notify_value_changed(&self) {
            let report = self.gamepad_values.lock().unwrap().get_report().clone();
            println!(
                "Report notify value changed, Hex: {}",
                report
                    .iter()
                    .map(|b| format!("{:02X}", b))
                    .collect::<Vec<_>>()
                    .join(" ")
            );
        }
    }
}

pub(crate) struct ReportChrcInterface(pub Arc<Mutex<ReportChrc>>);

#[gatt_characteristic()]
impl ReportChrcInterface {
    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<Vec<u8>> {
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
        _options: HashMap<String, String>,
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

    #[zbus(signal)]
    //async fn properties_changed(&self) -> zbus::Result<()>;

    pub async fn notify(&mut self, signal_emitter: SignalEmitter<'_>) -> zbus::Result<()> {
        let report = self
            .0
            .lock()
            .unwrap()
            .gamepad_values
            .lock()
            .unwrap()
            .get_report();

        println!(
            "Sending notification with values. Hex: {}",
            report
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ")
        );

        let mut changed = HashMap::new();
        changed.insert("Value", Value::from(report));

        let interface_name = InterfaceName::try_from("org.freedesktop.DBus.Properties").unwrap();
        // No invalidated properties.
        let invalidated_properties = Cow::Borrowed(&[] as &[_]);

        // Emit the signal.
        let result = Properties::properties_changed(
            &signal_emitter,
            interface_name,
            changed,
            invalidated_properties,
        )
        .await;

        println!("Properties::properties_changed result: {:?}", result);

        Ok(())
    }
}
