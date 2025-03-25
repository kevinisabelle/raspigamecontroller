use crate::bluez::advertisment::Advertisement;
use crate::constants::ADV_APPEARANCE_GAMEPAD;
use crate::gamepad_values::GamepadValues1;
use crate::hidimpl::battery_level_chrc::{BatteryLevelChrc, BatteryLevelChrcInterface};
use crate::hidimpl::battery_service::{BatteryService, BatteryServiceInterface};
use crate::hidimpl::ccc_desc::{CCCDescInterface, ClientCharacteristicConfigurationDesc};
use crate::hidimpl::device_info_service::{DeviceInfoService, DeviceInfoServiceInterface};
use crate::hidimpl::gatt_application::{GattApplication, GattApplicationInterface, ObjectManagerInterface};
use crate::hidimpl::hardware_revision_chrc::{HardwareRevisionChrc, HardwareRevisionChrcInterface};
use crate::hidimpl::hid_control_point_chrc::{HidControlPointChrc, HidControlPointChrcInterface};
use crate::hidimpl::hid_info_chrc::{HidInfoChrc, HidInfoChrcInterface};
use crate::hidimpl::hid_service::{HidService, HidServiceInterface};
use crate::hidimpl::manufacturer_name_chrc::{ManufacturerNameChrc, ManufacturerNameChrcInterface};
use crate::hidimpl::model_number_chrc::{ModelNumberChrc, ModelNumberChrcInterface};
use crate::hidimpl::pnpid_chrc::{PnpIdChrc, PnpIdChrcInterface};
use crate::hidimpl::protocol_mode_chrc::{ProtocolModeChrc, ProtocolModeChrcInterface};
use crate::hidimpl::report_chrc::{ReportChrc, ReportChrcInterface};
use crate::hidimpl::report_map_chrc::{ReportMapChrc, ReportMapChrcInterface};
use crate::hidimpl::report_ref_desc::{ReportReferenceDesc, ReportReferenceDescInterface};
use crate::hidimpl::serial_number_chrc::{SerialNumberChrc, SerialNumberChrcInterface};
use crate::utils::ObjectPathTrait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::object_server::Interface;
use zbus::{proxy, Connection, Error, Proxy};

pub fn create_advertisement(path: String) -> Advertisement {
    let adv = Advertisement::new(
        path,
        "peripheral".to_string(),
        Some(vec!["1812".to_string()]),
        None,
        None,
        None,
        Some("KiGP2".to_string()),
        true,
        None,
        Some(ADV_APPEARANCE_GAMEPAD),
    );
    adv
}

pub async fn create_and_register_application(
    connection: &Connection,
    gamepad_values: Arc<Mutex<GamepadValues1>>,
) -> zbus::Result<()> {
    println!("Creating GattApplication");

    let app = Arc::new(Mutex::new(GattApplication::new("/".to_string())));
    let app_interface = GattApplicationInterface(app.clone());
    let app_object_manager_interface = ObjectManagerInterface(app.clone());

    let hid_service = get_hid_service(connection, gamepad_values.clone()).await?;
    let battery_service = get_battery_service(connection).await?;
    let device_info_service = get_device_info_service(connection).await?;

    app.lock().unwrap().hid_service = Some(hid_service.clone());
    app.lock().unwrap().battery_service = Some(battery_service.clone());
    app.lock().unwrap().device_info_service = Some(device_info_service.clone());

    let app_object_path = app.lock().unwrap().object_path().clone();
    register_object(connection, app_object_path.clone(), app_interface).await?;
    register_object(connection, app_object_path.clone(), app_object_manager_interface).await?;
    register_application(connection, app_object_path.clone().as_str()).await?;

    Ok(())
}

#[proxy(
    interface = "org.bluez.GattManager1",
    default_service = "org.bluez",
    default_path = "/org/bluez/hci0"
)]
trait GattManager {
    fn register_application(
        &self,
        application: zbus::zvariant::ObjectPath<'_>,
        options: std::collections::HashMap<String, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;
}

pub async fn register_application(connection: &Connection, app_path: &str) -> zbus::Result<()> {
    let adapter_path = find_adapter(connection).await?;

    let gatt_manager: Proxy = Proxy::new(
        connection,
        "org.bluez",
        adapter_path,
        "org.bluez.GattManager1",
    )
    .await?;

    // Create an empty dictionary for the options.
    let options: HashMap<String, zbus::zvariant::Value> = HashMap::new();
    let app_object_path = zbus::zvariant::ObjectPath::try_from(app_path)?;

    // Call the RegisterApplication method.
    let result = gatt_manager
        .call_method("RegisterApplication", &(app_object_path, options))
        .await?;

    println!("Registered application: {:?}", result);

    Ok(())
}

async fn find_adapter(connection: &Connection) -> Result<String, Error> {
    let proxy = zbus::fdo::ObjectManagerProxy::new(connection, "org.bluez", "/").await?;
    let objects = proxy.get_managed_objects().await?;

    for (path, interfaces) in objects {
        if interfaces.contains_key("org.bluez.GattManager1") {
            println!("Found GattManager1 at path: {}", path);
            return Ok(path.to_string());
        }
    }

    panic!("No adapter found");
}

async fn register_object<T>(connection: &Connection, path: String, object: T) -> zbus::Result<()>
where
    T: Interface,
{
    let object_type = std::any::type_name::<T>();
    println!(
        "Registering object of type: {} at path {}",
        object_type,
        path.clone()
    );

    let result = match connection.object_server().at(path.clone(), object).await {
        Ok(_) => {
            println!("Registered object: {}", path);
            Ok(())
        }
        Err(e) => {
            println!("Error registering object: {}", e);
            Err(e)
        }
    };
    result
}

async fn get_device_info_service(
    connection: &Connection,
) -> zbus::Result<Arc<Mutex<DeviceInfoService>>> {
    let device_info_service = Arc::new(Mutex::new(DeviceInfoService::new(
        "/org/bluez/gamepadki/service1".to_string(),
    )));

    let device_info_service_path = device_info_service.lock().unwrap().object_path().clone();

    let manufacturer_name_chrc = Arc::new(Mutex::new(ManufacturerNameChrc::new(
        format!("{}/char0", device_info_service_path.clone()),
        device_info_service_path.clone(),
    )));

    let manufacturer_name_chrc_interface =
        ManufacturerNameChrcInterface(manufacturer_name_chrc.clone());
    let manufacturer_name_chrc_path = manufacturer_name_chrc.lock().unwrap().object_path().clone();

    register_object(
        connection,
        manufacturer_name_chrc_path.clone(),
        manufacturer_name_chrc_interface,
    )
    .await?;

    let model_number_chrc = Arc::new(Mutex::new(ModelNumberChrc::new(
        format!("{}/char1", device_info_service_path.clone()),
        device_info_service_path.clone(),
    )));

    let model_number_chrc_interface = ModelNumberChrcInterface(model_number_chrc.clone());
    let model_number_chrc_path = model_number_chrc.lock().unwrap().object_path().clone();
    register_object(
        connection,
        model_number_chrc_path.clone(),
        model_number_chrc_interface,
    )
    .await?;

    let serial_number_chrc = Arc::new(Mutex::new(SerialNumberChrc::new(
        format!("{}/char2", device_info_service_path.clone()),
        device_info_service_path.clone(),
    )));

    let serial_number_chrc_interface = SerialNumberChrcInterface(serial_number_chrc.clone());
    let serial_number_chrc_path = serial_number_chrc.lock().unwrap().object_path().clone();
    register_object(
        connection,
        serial_number_chrc_path.clone(),
        serial_number_chrc_interface,
    )
    .await?;

    let hardware_revision_chrc = Arc::new(Mutex::new(HardwareRevisionChrc::new(
        format!("{}/char3", device_info_service_path.clone()),
        device_info_service_path.clone(),
    )));

    let hardware_revision_chrc_interface =
        HardwareRevisionChrcInterface(hardware_revision_chrc.clone());
    let hardware_revision_chrc_path = hardware_revision_chrc.lock().unwrap().object_path().clone();
    register_object(
        connection,
        hardware_revision_chrc_path.clone(),
        hardware_revision_chrc_interface,
    )
    .await?;

    let pnp_id_chrc = Arc::new(Mutex::new(PnpIdChrc::new(
        format!("{}/char4", device_info_service_path.clone()),
        device_info_service_path.clone(),
    )));

    let pnp_id_chrc_interface = PnpIdChrcInterface(pnp_id_chrc.clone());
    let pnp_id_chrc_path = pnp_id_chrc.lock().unwrap().object_path().clone();
    register_object(connection, pnp_id_chrc_path.clone(), pnp_id_chrc_interface).await?;

    device_info_service
        .lock()
        .unwrap()
        .add_characteristic_path(manufacturer_name_chrc.lock().unwrap().object_path().clone());
    device_info_service
        .lock()
        .unwrap()
        .add_characteristic_path(model_number_chrc.lock().unwrap().object_path().clone());
    device_info_service
        .lock()
        .unwrap()
        .add_characteristic_path(serial_number_chrc.lock().unwrap().object_path().clone());
    device_info_service
        .lock()
        .unwrap()
        .add_characteristic_path(hardware_revision_chrc.lock().unwrap().object_path().clone());
    device_info_service
        .lock()
        .unwrap()
        .add_characteristic_path(pnp_id_chrc.lock().unwrap().object_path().clone());
    
    device_info_service.lock().unwrap().manufacturer_name_chrc = Some(manufacturer_name_chrc.clone());
    device_info_service.lock().unwrap().model_number_chrc = Some(model_number_chrc.clone());
    device_info_service.lock().unwrap().serial_number_chrc = Some(serial_number_chrc.clone());
    device_info_service.lock().unwrap().hardware_revision_chrc = Some(hardware_revision_chrc.clone());
    device_info_service.lock().unwrap().pnp_id_chrc = Some(pnp_id_chrc.clone());

    let device_info_service_interface = DeviceInfoServiceInterface(device_info_service.clone());
    let device_info_service_path = device_info_service.lock().unwrap().object_path().clone();
    register_object(
        connection,
        device_info_service_path.clone(),
        device_info_service_interface,
    )
    .await?;

    Ok(device_info_service)
}

async fn get_battery_service(connection: &Connection) -> Result<Arc<Mutex<BatteryService>>, Error> {
    let battery_service = Arc::new(Mutex::new(BatteryService::new(
        "/org/bluez/gamepadki/service2".to_string(),
    )));

    let battery_service_path = battery_service.lock().unwrap().object_path().clone();

    let battery_level_chrc = Arc::new(Mutex::new(BatteryLevelChrc::new(
        format!("{}/char0", battery_service_path.clone()),
        battery_service_path.clone(),
    )));
    
    battery_service.lock().unwrap().battery_level_chrc = Some(battery_level_chrc.clone());

    let battery_level_chrc_interface = BatteryLevelChrcInterface(battery_level_chrc.clone());
    let battery_level_chrc_path = battery_level_chrc.lock().unwrap().object_path().clone();

    register_object(
        connection,
        battery_level_chrc_path.clone(),
        battery_level_chrc_interface,
    )
    .await?;

    battery_service
        .lock()
        .unwrap()
        .add_characteristic_path(battery_level_chrc.lock().unwrap().object_path().clone());

    let battery_service_interface = BatteryServiceInterface(battery_service.clone());
    let battery_service_path = battery_service.lock().unwrap().object_path().clone();

    register_object(
        connection,
        battery_service_path.clone(),
        battery_service_interface,
    )
    .await?;

    Ok(battery_service)
}

async fn get_report_map_chrc(
    connection: &Connection,
    hid_service_path: String,
    gamepad_values: Arc<Mutex<GamepadValues1>>,
) -> Result<Arc<Mutex<ReportMapChrc>>, Error> {
    let report_map_chrc = Arc::new(Mutex::new(ReportMapChrc::new(
        format!("{}/char0", hid_service_path.clone()),
        hid_service_path.clone(),
        gamepad_values.clone(),
    )));
    let report_map_object_path = report_map_chrc.lock().unwrap().object_path().clone();
    let report_map_chrc_interface = ReportMapChrcInterface(report_map_chrc.clone());
    register_object(
        connection,
        report_map_object_path.clone(),
        report_map_chrc_interface,
    )
    .await?;

    Ok(report_map_chrc)
}

async fn get_report_chrc(
    connection: &Connection,
    hid_service_path: String,
    gamepad_values: Arc<Mutex<GamepadValues1>>,
) -> Result<Arc<Mutex<ReportChrc>>, Error> {
    let report_chrc = Arc::new(Mutex::new(ReportChrc::new(
        format!("{}/char1", hid_service_path.clone()),
        hid_service_path.clone(),
        gamepad_values,
    )));

    let report_chrc_interface = ReportChrcInterface(report_chrc.clone());

    let report_chrc_path = report_chrc.lock().unwrap().object_path().clone();

    let ccc_desc = Arc::new(Mutex::new(ClientCharacteristicConfigurationDesc::new(
        format!("{}/desc0", report_chrc_path.clone()),
        report_chrc_path.clone(),
    )));
    let rr_desc = Arc::new(Mutex::new(ReportReferenceDesc::new(
        format!("{}/desc1", report_chrc_path.clone()),
        report_chrc_path.clone(),
    )));

    let rr_desc_interface = ReportReferenceDescInterface(rr_desc.clone());
    let ccc_desc_interface = CCCDescInterface(ccc_desc.clone());

    let ccc_desc_path = ccc_desc.lock().unwrap().object_path().clone();
    let rr_desc_path = rr_desc.lock().unwrap().object_path().clone();

    report_chrc
        .lock()
        .unwrap()
        .add_descriptor_path(ccc_desc_path.clone());
    report_chrc
        .lock()
        .unwrap()
        .add_descriptor_path(rr_desc_path.clone());
    
    report_chrc.lock().unwrap().ccc_desc = Some(ccc_desc.clone());
    report_chrc.lock().unwrap().rr_desc = Some(rr_desc.clone());

    register_object(connection, ccc_desc_path.clone(), ccc_desc_interface).await?;
    register_object(connection, rr_desc_path.clone(), rr_desc_interface).await?;

    register_object(connection, report_chrc_path.clone(), report_chrc_interface).await?;

    Ok(report_chrc)
}

async fn get_protocol_mode_chrc(
    connection: &Connection,
    hid_service_path: String,
) -> Result<Arc<Mutex<ProtocolModeChrc>>, Error> {
    let protocol_mode_chrc = Arc::new(Mutex::new(ProtocolModeChrc::new(
        format!("{}/char2", hid_service_path.clone()),
        hid_service_path.clone(),
    )));
    let protocol_mode_object_path = protocol_mode_chrc.lock().unwrap().object_path().clone();
    let protocol_mode_chrc_interface = ProtocolModeChrcInterface(protocol_mode_chrc.clone());
    register_object(
        connection,
        protocol_mode_object_path.clone(),
        protocol_mode_chrc_interface,
    )
    .await?;

    Ok(protocol_mode_chrc)
}

async fn get_hid_info_chrc(
    connection: &Connection,
    hid_service_path: String,
) -> Result<Arc<Mutex<HidInfoChrc>>, Error> {
    let hid_info_chrc = Arc::new(Mutex::new(HidInfoChrc::new(
        format!("{}/char3", hid_service_path.clone()),
        hid_service_path.clone(),
    )));
    let hid_info_object_path = hid_info_chrc.lock().unwrap().object_path().clone();
    let hid_info_chrc_interface = HidInfoChrcInterface(hid_info_chrc.clone());
    register_object(
        connection,
        hid_info_object_path.clone(),
        hid_info_chrc_interface,
    )
    .await?;

    Ok(hid_info_chrc)
}

async fn get_hid_control_point(
    connection: &Connection,
    hid_service_path: String,
) -> Result<Arc<Mutex<HidControlPointChrc>>, Error> {
    let hid_control_point_chrc = Arc::new(Mutex::new(HidControlPointChrc::new(
        format!("{}/char4", hid_service_path.clone()),
        hid_service_path.clone(),
    )));
    let hid_control_point_object_path =
        hid_control_point_chrc.lock().unwrap().object_path().clone();
    let hid_control_point_chrc_interface =
        HidControlPointChrcInterface(hid_control_point_chrc.clone());
    register_object(
        connection,
        hid_control_point_object_path.clone(),
        hid_control_point_chrc_interface,
    )
    .await?;

    Ok(hid_control_point_chrc)
}

async fn get_hid_service(
    connection: &Connection,
    gamepad_values: Arc<Mutex<GamepadValues1>>,
) -> Result<Arc<Mutex<HidService>>, Error> {
    let hid_service = Arc::new(Mutex::new(HidService::new(
        "/org/bluez/gamepadki/service0".to_string(),
    )));
    let hid_service_path = hid_service.lock().unwrap().object_path().clone();

    let report_map_chrc =
        get_report_map_chrc(connection, hid_service_path.clone(), gamepad_values.clone()).await?;

    let report_chrc =
        get_report_chrc(connection, hid_service_path.clone(), gamepad_values.clone()).await?;

    let protocol_mode_chrc = get_protocol_mode_chrc(connection, hid_service_path.clone()).await?;

    let hid_control_point_chrc =
        get_hid_control_point(connection, hid_service_path.clone()).await?;

    let hid_info_chrc = get_hid_info_chrc(connection, hid_service_path.clone()).await?;

    hid_service
        .lock()
        .unwrap()
        .add_characteristic_path(report_map_chrc.lock().unwrap().object_path().clone());
    hid_service
        .lock()
        .unwrap()
        .add_characteristic_path(report_chrc.lock().unwrap().object_path().clone());
    hid_service
        .lock()
        .unwrap()
        .add_characteristic_path(protocol_mode_chrc.lock().unwrap().object_path().clone());
    hid_service
        .lock()
        .unwrap()
        .add_characteristic_path(hid_info_chrc.lock().unwrap().object_path().clone());
    hid_service
        .lock()
        .unwrap()
        .add_characteristic_path(hid_control_point_chrc.lock().unwrap().object_path().clone());

    hid_service.lock().unwrap().report_chrc = Some(report_chrc.clone());
    hid_service.lock().unwrap().reportmap_chrc = Some(report_map_chrc.clone());
    hid_service.lock().unwrap().protocol_mode_chrc = Some(protocol_mode_chrc.clone());
    hid_service.lock().unwrap().hid_info_chrc = Some(hid_info_chrc.clone());
    hid_service.lock().unwrap().hid_control_point_chrc = Some(hid_control_point_chrc.clone());
    
    let hid_service_interface = HidServiceInterface(hid_service.clone());

    connection
        .object_server()
        .at(hid_service_path.clone(), hid_service_interface)
        .await?;

    Ok(hid_service)
}
