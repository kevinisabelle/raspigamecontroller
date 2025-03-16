use std::sync::Arc;
use crate::bluez::register_advertisement;
use crate::bluez::register_agent;
use crate::constants::{ADVERT_PATH, AGENT_PATH, APP_PATH, DEVICE_PATH};
use zbus::{Connection, Result};
use crate::gamepad_values::GamepadValues1;
use crate::hid::{register_application};
use crate::utils::register_object;

mod bluez;
mod constants;
mod gamepad_values;
mod hid;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting GamepadKI...");

    let connection = Connection::system().await?;

    println!("Connection established!");

    let agent = Arc::new(bluez::Agent::new(AGENT_PATH.to_string()));
    register_object(&connection, agent).await?;
    register_agent(&connection, AGENT_PATH, "DisplayOnly").await?;
    
    println!("Creating advertisement...");

    let advert = Arc::new(hid::create_advertisement(ADVERT_PATH.to_string()));
    register_object(&connection, advert).await?;
    register_advertisement(&connection, ADVERT_PATH.to_string()).await?;

    println!("Advertisement registered!");
    
    let gamepad_values = Arc::from(GamepadValues1::new());
    let app = Arc::new(hid::GattApplication::new(APP_PATH.to_string(), DEVICE_PATH.to_string(), gamepad_values));
    register_application(&connection, app).await?;
    loop {}
}
