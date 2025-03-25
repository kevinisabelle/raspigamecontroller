use crate::bluez::advertisment::register_advertisement;
use crate::bluez::agent::{register_agent, Agent};
use crate::constants::{ADVERT_PATH, AGENT_PATH};
use crate::hid::create_and_register_application;
use crate::utils::register_object;
use std::sync::{Arc, Mutex};
use zbus::{Connection, Result};
use crate::gamepad_values::GamepadValues1;

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
    println!("Starting GamepadKI...");

    let connection = Connection::system().await?;

    println!("Connection established!");

    let agent = Arc::new(Agent::new(AGENT_PATH.to_string()));
    register_object(&connection, agent).await?;
    register_agent(&connection, AGENT_PATH, "DisplayOnly").await?;
        
    let gamepad_values = Arc::new(Mutex::new(GamepadValues1::new()));
    create_and_register_application(&connection, gamepad_values).await?;

    println!("Application registered!");

    println!("Creating advertisement...");

    let advert = Arc::new(hid::create_advertisement(ADVERT_PATH.to_string()));
    register_object(&connection, advert).await?;
    register_advertisement(&connection, ADVERT_PATH.to_string()).await?;

    println!("Advertisement registered!");
    
    println!("GamepadKI started! Waiting for gamepad connection...");
    loop {}
}
