// rust
use crate::bluez::{Advertisement, BaseGattCharacteristic, BaseGattDescriptor, BaseGattService};
use crate::constants::{
    BATTERY_SERVICE_UUID, DEVICE_INFORMATION_SERVICE_UUID, GATT_DESC_CLIENT_DESCRIPTOR_UUID,
    GATT_HID_CONTROL_POINT_UUID, GATT_HID_INFORMATION_UUID, GATT_PROTOCOL_MODE_UUID,
    GATT_REPORT_MAP_UUID, GATT_REPORT_UUID, GATT_SERVICE_HID_UUID,
    SERIAL_NUMBER_CHARACTERISTIC_UUID,
};
use crate::gamepad_values::GamepadValues1;
use crate::utils::{register_object, ObjectPathTrait};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use zbus::zvariant::Value;
use zbus::{interface, Connection};

// For the gamepad appearance constant, define a constant value. Adjust as needed.
const ADV_APPEARANCE_GAMEPAD: u16 = 0x03c0;

pub fn create_advertisement(path: String) -> Advertisement {
    let adv = Advertisement::new(
        path,
        "peripheral".to_string(),
        Option::from(vec!["1812".to_string()]),
        None,
        None,
        None,
        Option::from("KiGP".to_string()),
        true,
        None,
        Option::from(ADV_APPEARANCE_GAMEPAD),
    );
    adv
}

pub struct ReportMapChrc {
    pub base: BaseGattCharacteristic,
    pub gamepad_values: Arc<GamepadValues1>,
}

impl Debug for ReportMapChrc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReportMapChrc")
            .field("path", &self.base.path)
            .field("uuid", &self.base.uuid)
            .field("flags", &self.base.flags)
            .field("service", &self.base.service)
            .field("descriptors", &self.base.descriptors)
            .finish()
    }
}

impl ObjectPathTrait for ReportMapChrc {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

impl ReportMapChrc {
    pub fn new(path: String, service: String, gamepad_values: Arc<GamepadValues1>) -> Self {
        let uuid = GATT_REPORT_MAP_UUID.to_string();
        let flags = vec!["read".to_string()];
        Self {
            base: BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
            gamepad_values,
        }
    }

    pub fn get_properties(&self) -> HashMap<String, Value> {
        let mut properties = HashMap::new();
        properties.insert("Value".to_string(), self.get_value().into());
        properties.insert("Flags".to_string(), self.get_flags().into());
        properties.insert("UUID".to_string(), self.get_uuid().into());
        properties.insert("Service".to_string(), self.get_service().into());
        properties.insert("Descriptors".to_string(), self.get_descriptors().into());
        properties
    }
}

#[interface(name = "org.bluez.GattCharacteristic1")]
impl ReportMapChrc {
    #[zbus(property)]
    fn get_value(&self) -> Vec<u8> {
        self.gamepad_values.get_report_map()
    }

    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<Vec<u8>> {
        let report_map = self.gamepad_values.get_report_map();
        println!(
            "Report Map read handler called, Hex: {}",
            report_map
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ")
        );
        Ok(report_map)
    }

    #[zbus(property)]
    fn get_flags(&self) -> Vec<String> {
        self.base.flags.clone()
    }

    #[zbus(property)]
    fn get_uuid(&self) -> String {
        self.base.uuid.clone()
    }

    #[zbus(property)]
    fn get_service(&self) -> String {
        self.base.service.clone()
    }

    #[zbus(property)]
    fn get_descriptors(&self) -> Vec<String> {
        self.base.descriptors.clone()
    }
}

pub struct ClientCharacteristicConfigurationDesc {
    pub base: BaseGattDescriptor,
    pub value: Vec<u8>,
}

impl Debug for ClientCharacteristicConfigurationDesc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientCharacteristicConfigurationDesc")
            .field("path", &self.base.path)
            .field("uuid", &self.base.uuid)
            .field("flags", &self.base.flags)
            .field("characteristic", &self.base.characteristic)
            .finish()
    }
}

impl ObjectPathTrait for ClientCharacteristicConfigurationDesc {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

impl ClientCharacteristicConfigurationDesc {
    pub fn new(path: String, characteristic: String) -> Self {
        Self {
            base: BaseGattDescriptor::new(
                path,
                GATT_DESC_CLIENT_DESCRIPTOR_UUID.to_string(),
                vec!["read".to_string(), "write".to_string()],
                characteristic,
            ),
            value: vec![0x00, 0x00],
        }
    }

    pub fn set_value(&mut self, value: Vec<u8>) {
        self.value = value;
    }
}

#[interface(name = "org.bluez.GattDescriptor1")]
impl ClientCharacteristicConfigurationDesc {
    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<Vec<u8>> {
        Ok(self.value.clone())
    }

    fn write_value(
        &mut self,
        value: Vec<u8>,
        _options: HashMap<String, String>,
    ) -> zbus::fdo::Result<()> {
        println!(
            "Client Characteristic Configuration Descriptor write handler called, Hex: {}",
            value
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ")
        );
        self.set_value(value);
        Ok(())
    }

    #[zbus(property)]
    fn get_flags(&self) -> Vec<String> {
        self.base.flags.clone()
    }

    #[zbus(property)]
    fn get_uuid(&self) -> String {
        self.base.uuid.clone()
    }

    #[zbus(property)]
    fn get_characteristic(&self) -> String {
        self.base.characteristic.clone()
    }
}

pub struct ReportReferenceDesc {
    pub base: BaseGattDescriptor,
    pub value: Vec<u8>,
}

impl Debug for ReportReferenceDesc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReportReferenceDesc")
            .field("path", &self.base.path)
            .field("uuid", &self.base.uuid)
            .field("flags", &self.base.flags)
            .field("characteristic", &self.base.characteristic)
            .finish()
    }
}

impl ObjectPathTrait for ReportReferenceDesc {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

impl ReportReferenceDesc {
    pub fn new(path: String, characteristic: String) -> Self {
        Self {
            base: BaseGattDescriptor::new(
                path,
                GATT_DESC_CLIENT_DESCRIPTOR_UUID.to_string(),
                vec!["read".to_string()],
                characteristic,
            ),
            value: vec![0x00, 0x01],
        }
    }

    pub fn set_value(&mut self, value: Vec<u8>) {
        self.value = value;
    }
}

#[interface(name = "org.bluez.GattDescriptor1")]
impl ReportReferenceDesc {
    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<Vec<u8>> {
        Ok(self.value.clone())
    }

    #[zbus(property)]
    fn get_flags(&self) -> Vec<String> {
        self.base.flags.clone()
    }

    #[zbus(property)]
    fn get_uuid(&self) -> String {
        self.base.uuid.clone()
    }

    #[zbus(property)]
    fn get_characteristic(&self) -> String {
        self.base.characteristic.clone()
    }
}

pub struct ReportChrc {
    pub base: BaseGattCharacteristic,
    pub gamepad_values: Arc<GamepadValues1>,

    pub ccc_desc: Arc<ClientCharacteristicConfigurationDesc>,
    pub rr_desc: Arc<ReportReferenceDesc>,
}

impl Debug for ReportChrc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReportChrc")
            .field("path", &self.base.path)
            .field("uuid", &self.base.uuid)
            .field("flags", &self.base.flags)
            .field("service", &self.base.service)
            .field("descriptors", &self.base.descriptors)
            .finish()
    }
}

impl ObjectPathTrait for ReportChrc {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

impl ReportChrc {
    pub fn new(path: String, service: String, gamepad_values: Arc<GamepadValues1>) -> Self {
        let uuid = GATT_REPORT_UUID.to_string();
        let flags = vec!["read".to_string(), "write".to_string()];
        let ccc_desc = Arc::new(ClientCharacteristicConfigurationDesc::new(
            format!("{}/desc0", path.clone()),
            path.clone(),
        ));
        let rr_desc = Arc::new(ReportReferenceDesc::new(
            format!("{}/desc1", path),
            path.clone(),
        ));

        Self {
            base: BaseGattCharacteristic::new(
                path.clone(),
                uuid,
                flags,
                service,
                vec![ccc_desc.base.path.clone(), rr_desc.base.path.clone()],
            ),
            gamepad_values,
            ccc_desc,
            rr_desc,
        }
    }
}

#[interface(name = "org.bluez.GattCharacteristic1")]
impl ReportChrc {
    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<Vec<u8>> {
        let report = self.gamepad_values.get_report_map();
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

    #[zbus(property)]
    fn get_flags(&self) -> Vec<String> {
        self.base.flags.clone()
    }

    #[zbus(property)]
    fn get_uuid(&self) -> String {
        self.base.uuid.clone()
    }

    #[zbus(property)]
    fn get_service(&self) -> String {
        self.base.service.clone()
    }

    #[zbus(property)]
    fn get_descriptors(&self) -> Vec<String> {
        self.base.descriptors.clone()
    }
}

pub struct ProtocolModeChrc {
    pub base: BaseGattCharacteristic,
    pub value: Vec<u8>,
}

impl Debug for ProtocolModeChrc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProtocolModeChrc")
            .field("path", &self.base.path)
            .field("uuid", &self.base.uuid)
            .field("flags", &self.base.flags)
            .field("service", &self.base.service)
            .field("descriptors", &self.base.descriptors)
            .finish()
    }
}

impl ObjectPathTrait for ProtocolModeChrc {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

impl ProtocolModeChrc {
    pub fn new(path: String, service: String) -> Self {
        let uuid = GATT_PROTOCOL_MODE_UUID.to_string();
        let flags = vec!["read".to_string(), "write".to_string()];
        Self {
            base: BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
            value: vec![0x01],
        }
    }
}

#[interface(name = "org.bluez.GattCharacteristic1")]
impl ProtocolModeChrc {
    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<Vec<u8>> {
        Ok(self.value.clone())
    }

    fn write_value(
        &mut self,
        value: Vec<u8>,
        _options: HashMap<String, String>,
    ) -> zbus::fdo::Result<()> {
        println!(
            "Protocol Mode write handler called, Hex: {}",
            value
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ")
        );
        self.value = value;
        Ok(())
    }

    #[zbus(property)]
    fn get_flags(&self) -> Vec<String> {
        self.base.flags.clone()
    }

    #[zbus(property)]
    fn get_uuid(&self) -> String {
        self.base.uuid.clone()
    }

    #[zbus(property)]
    fn get_service(&self) -> String {
        self.base.service.clone()
    }

    #[zbus(property)]
    fn get_descriptors(&self) -> Vec<String> {
        self.base.descriptors.clone()
    }
}

pub struct HidInfoChrc {
    pub base: BaseGattCharacteristic,
    pub value: Vec<u8>,
}

impl Debug for HidInfoChrc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HidInfoChrc")
            .field("path", &self.base.path)
            .field("uuid", &self.base.uuid)
            .field("flags", &self.base.flags)
            .field("service", &self.base.service)
            .field("descriptors", &self.base.descriptors)
            .finish()
    }
}

impl ObjectPathTrait for HidInfoChrc {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

impl HidInfoChrc {
    pub fn new(path: String, service: String) -> Self {
        let uuid = GATT_HID_INFORMATION_UUID.to_string();
        let flags = vec!["read".to_string()];
        Self {
            base: BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
            value: vec![0x11, 0x01, 0x00, 0x03], // bcdHID, bCountryCode, Flags (RemoteWake, NormallyConnectable)
        }
    }
}

#[interface(name = "org.bluez.GattCharacteristic1")]
impl HidInfoChrc {
    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<Vec<u8>> {
        Ok(self.value.clone())
    }

    #[zbus(property)]
    fn get_flags(&self) -> Vec<String> {
        self.base.flags.clone()
    }

    #[zbus(property)]
    fn get_uuid(&self) -> String {
        self.base.uuid.clone()
    }

    #[zbus(property)]
    fn get_service(&self) -> String {
        self.base.service.clone()
    }

    #[zbus(property)]
    fn get_descriptors(&self) -> Vec<String> {
        self.base.descriptors.clone()
    }
}

pub struct HidControlPointChrc {
    pub base: BaseGattCharacteristic,
}

impl Debug for HidControlPointChrc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HidControlPointChrc")
            .field("path", &self.base.path)
            .field("uuid", &self.base.uuid)
            .field("flags", &self.base.flags)
            .field("service", &self.base.service)
            .field("descriptors", &self.base.descriptors)
            .finish()
    }
}

impl ObjectPathTrait for HidControlPointChrc {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

impl HidControlPointChrc {
    pub fn new(path: String, service: String) -> Self {
        let uuid = GATT_HID_CONTROL_POINT_UUID.to_string();
        let flags = vec!["write".to_string()];
        Self {
            base: BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
        }
    }
}

#[interface(name = "org.bluez.GattCharacteristic1")]
impl HidControlPointChrc {
    fn write_value(
        &mut self,
        value: Vec<u8>,
        _options: HashMap<String, String>,
    ) -> zbus::fdo::Result<()> {
        println!(
            "HID Control Point write handler called, Hex: {}",
            value
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ")
        );
        Ok(())
    }

    #[zbus(property)]
    fn get_flags(&self) -> Vec<String> {
        self.base.flags.clone()
    }

    #[zbus(property)]
    fn get_uuid(&self) -> String {
        self.base.uuid.clone()
    }

    #[zbus(property)]
    fn get_service(&self) -> String {
        self.base.service.clone()
    }

    #[zbus(property)]
    fn get_descriptors(&self) -> Vec<String> {
        self.base.descriptors.clone()
    }
}

pub struct HidService {
    pub base: BaseGattService,
    pub report_map_chrc: Arc<ReportMapChrc>,
    pub report_chrc: Arc<ReportChrc>,
    pub protocol_mode_chrc: Arc<ProtocolModeChrc>,
    pub hid_info_chrc: Arc<HidInfoChrc>,
    pub hid_control_point_chrc: Arc<HidControlPointChrc>,
}

impl ObjectPathTrait for HidService {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

impl Debug for HidService {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HidService")
            .field("path", &self.base.path)
            .field("uuid", &self.base.uuid)
            .field("primary", &self.base.primary)
            .field("characteristics", &self.base.characteristics)
            .finish()
    }
}

impl HidService {
    pub fn new(path: String, device: String, gamepad_values: Arc<GamepadValues1>) -> Self {
        let report_map_chrc = Arc::new(ReportMapChrc::new(
            format!("{}/char0", path.clone()),
            path.clone(),
            gamepad_values.clone(),
        ));
        let report_chrc = Arc::new(ReportChrc::new(
            format!("{}/char1", path.clone()),
            path.clone(),
            gamepad_values,
        ));
        let protocol_mode_chrc = Arc::new(ProtocolModeChrc::new(
            format!("{}/char2", path.clone()),
            path.clone(),
        ));
        let hid_info_chrc = Arc::new(HidInfoChrc::new(
            format!("{}/char3", path.clone()),
            path.clone(),
        ));
        let hid_control_point_chrc = Arc::new(HidControlPointChrc::new(
            format!("{}/char4", path.clone()),
            path.clone(),
        ));

        Self {
            base: BaseGattService::new(
                path,
                GATT_SERVICE_HID_UUID.to_string(),
                true,
                vec![
                    report_map_chrc.base.path.clone(),
                    report_chrc.base.path.clone(),
                    protocol_mode_chrc.base.path.clone(),
                    hid_info_chrc.base.path.clone(),
                    hid_control_point_chrc.base.path.clone(),
                ],
            ),
            report_map_chrc,
            report_chrc,
            protocol_mode_chrc,
            hid_info_chrc,
            hid_control_point_chrc,
        }
    }
}

#[interface(name = "org.bluez.GattService1")]
impl HidService {
    #[zbus(property)]
    fn get_primary(&self) -> bool {
        self.base.primary
    }

    #[zbus(property)]
    fn get_uuid(&self) -> String {
        self.base.uuid.clone()
    }

    #[zbus(property)]
    fn get_characteristics(&self) -> Vec<String> {
        vec![
            self.report_map_chrc.base.path.clone(),
            self.report_chrc.base.path.clone(),
            self.protocol_mode_chrc.base.path.clone(),
            self.hid_info_chrc.base.path.clone(),
            self.hid_control_point_chrc.base.path.clone(),
        ]
    }
}

pub struct ManufacturerNameChrc {
    pub base: BaseGattCharacteristic,
    pub value: Vec<u8>,
}

impl Debug for ManufacturerNameChrc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ManufacturerNameChrc")
            .field("path", &self.base.path)
            .field("uuid", &self.base.uuid)
            .field("flags", &self.base.flags)
            .field("service", &self.base.service)
            .field("descriptors", &self.base.descriptors)
            .finish()
    }
}

impl ObjectPathTrait for ManufacturerNameChrc {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

impl ManufacturerNameChrc {
    pub fn new(path: String, service: String) -> Self {
        let uuid = "2a29".to_string();
        let flags = vec!["read".to_string()];
        Self {
            base: BaseGattCharacteristic::new(path.clone(), uuid, flags, service, vec![]),
            value: "Ki".as_bytes().to_vec(),
        }
    }
}

#[interface(name = "org.bluez.GattCharacteristic1")]
impl ManufacturerNameChrc {
    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<Vec<u8>> {
        Ok(self.value.clone())
    }

    #[zbus(property)]
    fn get_flags(&self) -> Vec<String> {
        self.base.flags.clone()
    }

    #[zbus(property)]
    fn get_uuid(&self) -> String {
        self.base.uuid.clone()
    }

    #[zbus(property)]
    fn get_service(&self) -> String {
        self.base.service.clone()
    }

    #[zbus(property)]
    fn get_descriptors(&self) -> Vec<String> {
        self.base.descriptors.clone()
    }
}

pub struct ModelNumberChrc {
    pub base: BaseGattCharacteristic,
    pub value: Vec<u8>,
}

impl Debug for ModelNumberChrc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModelNumberChrc")
            .field("path", &self.base.path)
            .field("uuid", &self.base.uuid)
            .field("flags", &self.base.flags)
            .field("service", &self.base.service)
            .field("descriptors", &self.base.descriptors)
            .finish()
    }
}

impl ObjectPathTrait for ModelNumberChrc {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

impl ModelNumberChrc {
    pub fn new(path: String, service: String) -> Self {
        let uuid = "2a24".to_string();
        let flags = vec!["read".to_string()];
        Self {
            base: BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
            value: "GP".as_bytes().to_vec(),
        }
    }
}

#[interface(name = "org.bluez.GattCharacteristic1")]
impl ModelNumberChrc {
    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<Vec<u8>> {
        Ok(self.value.clone())
    }

    #[zbus(property)]
    fn get_flags(&self) -> Vec<String> {
        self.base.flags.clone()
    }

    #[zbus(property)]
    fn get_uuid(&self) -> String {
        self.base.uuid.clone()
    }

    #[zbus(property)]
    fn get_service(&self) -> String {
        self.base.service.clone()
    }

    #[zbus(property)]
    fn get_descriptors(&self) -> Vec<String> {
        self.base.descriptors.clone()
    }
}

pub struct SerialNumberChrc {
    pub base: BaseGattCharacteristic,
    pub value: Vec<u8>,
}

impl Debug for SerialNumberChrc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SerialNumberChrc")
            .field("path", &self.base.path)
            .field("uuid", &self.base.uuid)
            .field("flags", &self.base.flags)
            .field("service", &self.base.service)
            .field("descriptors", &self.base.descriptors)
            .finish()
    }
}

impl ObjectPathTrait for SerialNumberChrc {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

impl SerialNumberChrc {
    pub fn new(path: String, service: String) -> Self {
        let uuid = SERIAL_NUMBER_CHARACTERISTIC_UUID.to_string();
        let flags = vec!["read".to_string()];
        Self {
            base: BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
            value: "1.0".as_bytes().to_vec(),
        }
    }
}

#[interface(name = "org.bluez.GattCharacteristic1")]
impl SerialNumberChrc {
    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<Vec<u8>> {
        Ok(self.value.clone())
    }

    #[zbus(property)]
    fn get_flags(&self) -> Vec<String> {
        self.base.flags.clone()
    }

    #[zbus(property)]
    fn get_uuid(&self) -> String {
        self.base.uuid.clone()
    }

    #[zbus(property)]
    fn get_service(&self) -> String {
        self.base.service.clone()
    }

    #[zbus(property)]
    fn get_descriptors(&self) -> Vec<String> {
        self.base.descriptors.clone()
    }
}

pub struct HardwareRevisionChrc {
    pub base: BaseGattCharacteristic,
    pub value: Vec<u8>,
}

impl Debug for HardwareRevisionChrc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HardwareRevisionChrc")
            .field("path", &self.base.path)
            .field("uuid", &self.base.uuid)
            .field("flags", &self.base.flags)
            .field("service", &self.base.service)
            .field("descriptors", &self.base.descriptors)
            .finish()
    }
}

impl ObjectPathTrait for HardwareRevisionChrc {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

impl HardwareRevisionChrc {
    pub fn new(path: String, service: String) -> Self {
        let uuid = "2a27".to_string();
        let flags = vec!["read".to_string()];
        Self {
            base: BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
            value: "1.0".as_bytes().to_vec(),
        }
    }
}

#[interface(name = "org.bluez.GattCharacteristic1")]
impl HardwareRevisionChrc {
    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<Vec<u8>> {
        Ok(self.value.clone())
    }

    #[zbus(property)]
    fn get_flags(&self) -> Vec<String> {
        self.base.flags.clone()
    }

    #[zbus(property)]
    fn get_uuid(&self) -> String {
        self.base.uuid.clone()
    }

    #[zbus(property)]
    fn get_service(&self) -> String {
        self.base.service.clone()
    }

    #[zbus(property)]
    fn get_descriptors(&self) -> Vec<String> {
        self.base.descriptors.clone()
    }
}

pub struct PnpIdChrc {
    pub base: BaseGattCharacteristic,
}

impl Debug for PnpIdChrc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PnpIdChrc")
            .field("path", &self.base.path)
            .field("uuid", &self.base.uuid)
            .field("flags", &self.base.flags)
            .field("service", &self.base.service)
            .field("descriptors", &self.base.descriptors)
            .finish()
    }
}

impl ObjectPathTrait for PnpIdChrc {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

impl PnpIdChrc {
    pub fn new(path: String, service: String) -> Self {
        let uuid = "2a50".to_string();
        let flags = vec!["read".to_string()];
        Self {
            base: BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
        }
    }
}

#[interface(name = "org.bluez.GattCharacteristic1")]
impl PnpIdChrc {
    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<Vec<u8>> {
        Ok(vec![
            0x0, 0x2, 0x6, 0xD, 0x0, 0x4, 0x7, 0x8, 0x5, 0x6, 0x0, 0x0, 0x0, 0x1,
        ])
    }

    #[zbus(property)]
    fn get_flags(&self) -> Vec<String> {
        self.base.flags.clone()
    }

    #[zbus(property)]
    fn get_uuid(&self) -> String {
        self.base.uuid.clone()
    }

    #[zbus(property)]
    fn get_service(&self) -> String {
        self.base.service.clone()
    }

    #[zbus(property)]
    fn get_descriptors(&self) -> Vec<String> {
        self.base.descriptors.clone()
    }
}

pub struct DeviceInfoService {
    pub base: BaseGattService,
    pub manufacturer_name_chrc: Arc<ManufacturerNameChrc>,
    pub model_number_chrc: Arc<ModelNumberChrc>,
    pub serial_number_chrc: Arc<SerialNumberChrc>,
    pub hardware_revision_chrc: Arc<HardwareRevisionChrc>,
    pub pnp_id_chrc: Arc<PnpIdChrc>,
}

impl Debug for DeviceInfoService {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeviceInfoService")
            .field("path", &self.base.path)
            .field("uuid", &self.base.uuid)
            .field("primary", &self.base.primary)
            .field("characteristics", &self.base.characteristics)
            .finish()
    }
}

impl ObjectPathTrait for DeviceInfoService {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

impl DeviceInfoService {
    pub fn new(path: String, device: String) -> Self {
        let manufacturer_name_chrc = Arc::new(ManufacturerNameChrc::new(
            format!("{}/char0", path.clone()),
            path.clone(),
        ));
        let model_number_chrc = Arc::new(ModelNumberChrc::new(
            format!("{}/char1", path.clone()),
            path.clone(),
        ));
        let serial_number_chrc = Arc::new(SerialNumberChrc::new(
            format!("{}/char2", path.clone()),
            path.clone(),
        ));
        let hardware_revision_chrc = Arc::new(HardwareRevisionChrc::new(
            format!("{}/char3", path.clone()),
            path.clone(),
        ));
        let pnp_id_chrc = Arc::new(PnpIdChrc::new(
            format!("{}/char4", path.clone()),
            path.clone(),
        ));

        Self {
            base: BaseGattService::new(
                path,
                DEVICE_INFORMATION_SERVICE_UUID.to_string(),
                true,
                vec![
                    manufacturer_name_chrc.base.path.clone(),
                    model_number_chrc.base.path.clone(),
                    serial_number_chrc.base.path.clone(),
                    hardware_revision_chrc.base.path.clone(),
                    pnp_id_chrc.base.path.clone(),
                ],
            ),
            manufacturer_name_chrc,
            model_number_chrc,
            serial_number_chrc,
            hardware_revision_chrc,
            pnp_id_chrc,
        }
    }
}

#[interface(name = "org.bluez.GattService1")]
impl DeviceInfoService {
    #[zbus(property)]
    fn get_primary(&self) -> bool {
        self.base.primary
    }

    #[zbus(property)]
    fn get_uuid(&self) -> String {
        self.base.uuid.clone()
    }

    #[zbus(property)]
    fn get_characteristics(&self) -> Vec<String> {
        vec![
            self.manufacturer_name_chrc.base.path.clone(),
            self.model_number_chrc.base.path.clone(),
            self.serial_number_chrc.base.path.clone(),
            self.hardware_revision_chrc.base.path.clone(),
            self.pnp_id_chrc.base.path.clone(),
        ]
    }
}

pub struct BatteryLevelChrc {
    pub base: BaseGattCharacteristic,
    pub value: u8,
}

impl Debug for BatteryLevelChrc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BatteryLevelChrc")
            .field("path", &self.base.path)
            .field("uuid", &self.base.uuid)
            .field("flags", &self.base.flags)
            .field("service", &self.base.service)
            .field("value", &self.value)
            .finish()
    }
}

impl ObjectPathTrait for BatteryLevelChrc {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

impl BatteryLevelChrc {
    pub fn new(path: String, service: String) -> Self {
        let uuid = "2a19".to_string();
        let flags = vec!["read".to_string()];
        Self {
            base: BaseGattCharacteristic::new(path, uuid, flags, service, vec![]),
            value: 100,
        }
    }
}

#[interface(name = "org.bluez.GattCharacteristic1")]
impl BatteryLevelChrc {
    fn read_value(&self, _options: HashMap<String, String>) -> zbus::fdo::Result<u8> {
        Ok(self.value)
    }

    #[zbus(property)]
    fn get_flags(&self) -> Vec<String> {
        self.base.flags.clone()
    }

    #[zbus(property)]
    fn get_uuid(&self) -> String {
        self.base.uuid.clone()
    }

    #[zbus(property)]
    fn get_service(&self) -> String {
        self.base.service.clone()
    }

    #[zbus(property)]
    fn get_descriptors(&self) -> Vec<String> {
        self.base.descriptors.clone()
    }
}

pub struct BatteryService {
    pub base: BaseGattService,
    pub battery_level_chrc: Arc<BatteryLevelChrc>,
}

impl Debug for BatteryService {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BatteryService")
            .field("path", &self.base.path)
            .field("uuid", &self.base.uuid)
            .field("primary", &self.base.primary)
            .field("characteristics", &self.base.characteristics)
            .finish()
    }
}

impl ObjectPathTrait for BatteryService {
    fn object_path(&self) -> String {
        self.base.path.to_string()
    }
}

impl BatteryService {
    pub fn new(path: String, device: String) -> Self {
        let battery_level_chrc = Arc::new(BatteryLevelChrc::new(
            format!("{}/char0", path.clone()),
            path.clone(),
        ));

        Self {
            base: BaseGattService::new(
                path,
                BATTERY_SERVICE_UUID.to_string(),
                true,
                vec![battery_level_chrc.base.path.clone()],
            ),
            battery_level_chrc,
        }
    }
}

#[interface(name = "org.bluez.GattService1")]
impl BatteryService {
    #[zbus(property)]
    fn get_primary(&self) -> bool {
        self.base.primary
    }

    #[zbus(property)]
    fn get_uuid(&self) -> String {
        self.base.uuid.clone()
    }

    #[zbus(property)]
    fn get_characteristics(&self) -> Vec<String> {
        vec![self.battery_level_chrc.base.path.clone()]
    }
}

pub struct GattApplication {
    pub hid_service: Arc<HidService>,
    pub device_info_service: Arc<DeviceInfoService>,
    pub battery_service: Arc<BatteryService>,
    pub path: String,
}

impl ObjectPathTrait for GattApplication {
    fn object_path(&self) -> String {
        self.path.to_string()
    }
}

impl Debug for GattApplication {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GattApplication {{ path: {} }}", self.path)
    }
}

impl GattApplication {
    pub fn new(path: String, device: String, gamepad_values: Arc<GamepadValues1>) -> Self {
        let hid_service = Arc::new(HidService::new(
            "/service0".to_string(),
            device.clone(),
            gamepad_values.clone(),
        ));
        let device_info_service = Arc::new(DeviceInfoService::new(
            "/service1".to_string(),
            device.clone(),
        ));
        let battery_service = Arc::new(BatteryService::new(
            "/service2".to_string(),
            device.to_string(),
        ));

        Self {
            hid_service,
            device_info_service,
            battery_service,
            path,
        }
    }
}

#[interface(name = "org.bluez.GattApplication1")]
impl GattApplication {
    // out_signature='a{oa{sa{sv}}}')
    pub fn get_managed_objects(&self) -> HashMap<String, HashMap<String, HashMap<String, Value>>> {
        let mut objects = HashMap::new();

        let mut hid_service_map = HashMap::new();

        let report_map_chrc_map = self.hid_service.report_map_chrc.get_properties();

        hid_service_map.insert(
            self.hid_service.report_map_chrc.base.path.clone(),
            report_map_chrc_map,
        );

        objects.insert(self.hid_service.base.path.clone(), hid_service_map);

        objects
    }
}

pub async fn register_application(
    connection: &Connection,
    app: Arc<GattApplication>,
) -> zbus::Result<()> {
    register_object(
        connection,
        app.clone().hid_service.clone().report_map_chrc.clone(),
    )
    .await?;
    /*register_object(connection, app.hid_service.report_chrc.ccc_desc.clone()).await?;
    register_object(connection, app.hid_service.report_chrc.rr_desc.clone()).await?;
    register_object(connection, app.hid_service.report_chrc.clone()).await?;

    register_object(connection, app.hid_service.protocol_mode_chrc.clone()).await?;
    register_object(connection, app.hid_service.hid_info_chrc.clone()).await?;
    register_object(connection, app.hid_service.hid_control_point_chrc.clone()).await?;
    register_object(connection, app.hid_service.clone()).await?;*/

    /*register_object(connection, app.device_info_service.clone()).await?;
    register_object(
        connection,
        app.device_info_service.manufacturer_name_chrc.clone(),
    )
    .await?;
    register_object(
        connection,
        app.device_info_service.model_number_chrc.clone(),
    )
    .await?;
    register_object(
        connection,
        app.device_info_service.serial_number_chrc.clone(),
    )
    .await?;
    register_object(
        connection,
        app.device_info_service.hardware_revision_chrc.clone(),
    )
    .await?;
    register_object(connection, app.device_info_service.pnp_id_chrc.clone()).await?;

    register_object(connection, app.battery_service.clone()).await?;
    register_object(connection, app.battery_service.battery_level_chrc.clone()).await?;*/
    register_object(connection, app.clone()).await?;
    Ok(())
}
