use std::env;
use crate::bluez::advertisment::register_advertisement;
use crate::bluez::agent::{register_agent, Agent};
use crate::constants::{ADVERT_PATH, AGENT_PATH};
use crate::gamepad_updater::GamepadUpdater;
use crate::gamepad_values::GamepadValues1;
use crate::hardware::init_hardware;
use crate::hid::create_and_register_application;
use crate::utils::register_object;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use zbus::{Connection, Result};

mod bluez;
mod constants;
mod gamepad_updater;
mod gamepad_values;
mod hardware;
mod hid;
mod hidimpl;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {

    let mut args = env::args();

    let ms_timeout = args.nth(1).unwrap_or("20".to_string()).parse::<u64>().unwrap();
    
    println!("Starting GamepadKI... Refreshing every {}ms", ms_timeout);

    let _ = init_hardware();

    let connection = Connection::system().await?;

    println!("Connection established!");

    let agent = Arc::new(Agent::new(AGENT_PATH.to_string()));
    register_object(&connection, agent).await?;
    register_agent(&connection, AGENT_PATH, "KeyboardDisplay").await?;

    let gamepad_values = Arc::new(Mutex::new(GamepadValues1::new()));
    let app = create_and_register_application(&connection, gamepad_values.clone()).await?;

    println!("Application registered!");

    println!("Creating advertisement...");

    let advert = Arc::new(hid::create_advertisement(ADVERT_PATH.to_string()));
    register_object(&connection, advert).await?;
    register_advertisement(&connection, ADVERT_PATH.to_string()).await?;

    println!("Advertisement registered!");

    let mut updater_service =
        GamepadUpdater::new(gamepad_values.clone(), app, Duration::from_millis(ms_timeout));
    updater_service.start();

    println!("GamepadKI started! Waiting for gamepad connection...");
    loop {}
}
