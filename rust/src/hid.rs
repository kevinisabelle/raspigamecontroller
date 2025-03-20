use crate::bluez::advertisment::Advertisement;
use crate::constants::ADV_APPEARANCE_GAMEPAD;
use crate::gamepad_values::GamepadValues1;
use crate::hidimpl::ccc_desc::{CCCDescInterface, ClientCharacteristicConfigurationDesc};
use crate::hidimpl::gatt_application::GattApplication;
use crate::hidimpl::hid_service::HidService;
use crate::hidimpl::report_chrc::{ReportChrc, ReportChrcInterface};
use crate::hidimpl::report_map_chrc::{ReportMapChrc, ReportMapChrcInterface};
use crate::hidimpl::report_ref_desc::ReportReferenceDesc;
use crate::utils::ObjectPathTrait;
use std::sync::{Arc, Mutex};
use zbus::Connection;

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

pub async fn create_and_register_application(
    connection: &Connection,
    gamepad_values: Arc<GamepadValues1>,
) -> zbus::Result<()> {
    let mut app = GattApplication::new("/app".to_string());

    let hid_service = HidService::new("/service0".to_string());

    let hid_service_path = hid_service.object_path();

    let report_map_chrc = Arc::new(Mutex::new(ReportMapChrc::new(
        format!("{}/char0", hid_service_path.clone()),
        hid_service_path.clone(),
        gamepad_values.clone(),
    )));
    let report_map_object_path = report_map_chrc.lock().unwrap().object_path().clone();
    let report_map_chrc_interface = ReportMapChrcInterface(report_map_chrc);
    connection
        .object_server()
        .at(report_map_object_path, report_map_chrc_interface)
        .await?;

    let mut report_chrc = ReportChrc::new(
        format!("{}/char1", hid_service_path.clone()),
        hid_service_path.clone(),
        gamepad_values,
    );

    let report_chrc_path = report_chrc.object_path().clone();

    let ccc_desc = ClientCharacteristicConfigurationDesc::new(
        format!("{}/desc0", report_chrc_path.clone()),
        report_chrc_path.clone(),
    );
    let rr_desc = Arc::new(ReportReferenceDesc::new(
        format!("{}/desc1", report_chrc_path.clone()),
        report_chrc_path.clone(),
    ));

    report_chrc.add_descriptor_path(ccc_desc.object_path());
    report_chrc.add_descriptor_path(rr_desc.object_path());

    let ccc_desc_path = ccc_desc.object_path().clone();
    let ccc_desc_interface = CCCDescInterface(Arc::new(Mutex::new(ccc_desc)));

    let report_chrc_interface = ReportChrcInterface(Arc::new(Mutex::new(report_chrc)));
    connection
        .object_server()
        .at(report_chrc_path, report_chrc_interface)
        .await?;

    /*     let protocol_mode_chrc = Arc::new(ProtocolModeChrc::new(
            format!("{}/char2", hid_service_path.clone()),
            hid_service_path.clone(),
        ));
        let hid_info_chrc = Arc::new(HidInfoChrc::new(
            format!("{}/char3", hid_service_path.clone()),
            hid_service_path.clone(),
        ));
        let hid_control_point_chrc = Arc::new(HidControlPointChrc::new(
            format!("{}/char4", hid_service_path.clone()),
            hid_service_path.clone(),
        ));

        hid_service.add_characteristic_path(report_map_chrc.object_path());
        hid_service.add_characteristic_path(report_chrc.object_path());
        hid_service.add_characteristic_path(protocol_mode_chrc.object_path());
        hid_service.add_characteristic_path(hid_info_chrc.object_path());
        hid_service.add_characteristic_path(hid_control_point_chrc.object_path());

        let mut device_info_service = DeviceInfoService::new("/service1".to_string());

        let device_into_service_path = device_info_service.object_path();

        let manufacturer_name_chrc = Arc::new(ManufacturerNameChrc::new(
            format!("{}/char0", device_into_service_path.clone()),
            device_into_service_path.clone(),
        ));
        let model_number_chrc = Arc::new(ModelNumberChrc::new(
            format!("{}/char1", device_into_service_path.clone()),
            device_into_service_path.clone(),
        ));
        let serial_number_chrc = Arc::new(SerialNumberChrc::new(
            format!("{}/char2", device_into_service_path.clone()),
            device_into_service_path.clone(),
        ));
        let hardware_revision_chrc = Arc::new(HardwareRevisionChrc::new(
            format!("{}/char3", device_into_service_path.clone()),
            device_into_service_path.clone(),
        ));
        let pnp_id_chrc = Arc::new(PnpIdChrc::new(
            format!("{}/char4", device_into_service_path.clone()),
            device_into_service_path.clone(),
        ));

        device_info_service.add_characteristic_path(manufacturer_name_chrc.object_path());
        device_info_service.add_characteristic_path(model_number_chrc.object_path());
        device_info_service.add_characteristic_path(serial_number_chrc.object_path());
        device_info_service.add_characteristic_path(hardware_revision_chrc.object_path());
        device_info_service.add_characteristic_path(pnp_id_chrc.object_path());

        let mut battery_service = BatteryService::new("/service2".to_string());

        let battery_service_path = battery_service.object_path().clone();

        let battery_level_chrc = Arc::new(BatteryLevelChrc::new(
            format!("{}/char0", battery_service_path.clone()),
            battery_service_path.clone(),
        ));

        battery_service.add_characteristic_path(battery_level_chrc.object_path());

        //-- Register all objects

        // register_object(connection, report_map_chrc.clone()).await?;
        /*register_object(connection, Arc::new(report_chrc)).await?;
        register_object(connection, ccc_desc.clone()).await?;
        register_object(connection, rr_desc.clone()).await?;
        register_object(connection, protocol_mode_chrc.clone()).await?;
        register_object(connection, hid_info_chrc.clone()).await?;
        register_object(connection, hid_control_point_chrc.clone()).await?;

        register_object(connection, manufacturer_name_chrc.clone()).await?;
        register_object(connection, model_number_chrc.clone()).await?;
        register_object(connection, serial_number_chrc.clone()).await?;
        register_object(connection, hardware_revision_chrc.clone()).await?;
        register_object(connection, pnp_id_chrc.clone()).await?;
        register_object(connection, Arc::new(device_info_service)).await?;

        register_object(connection, battery_level_chrc.clone()).await?;
        register_object(connection, Arc::new(battery_service)).await?;

        app.hid_service = Some(&mut hid_service);

        register_object(connection, Arc::new(hid_service)).await?;

        register_object(connection, Arc::new(app)).await?;*/
    */
    Ok(())
}
