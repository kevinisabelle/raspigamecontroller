use zbus::{Connection, Result};
use crate::bluez::register_advertisement;
use crate::bluez::register_agent;

mod bluez;
mod hid;
mod utils;

const AGENT_PATH: &str = "/org/bluez/gamepadkiagent";

#[tokio::main]
async fn main() -> Result<()> {

    println!("Starting GamepadKI...");

    let connection = Connection::system()
        .await?;

    println!("Connection established!");

    let agent = bluez::Agent::new(AGENT_PATH.to_string());

    println!("Registering agent with path {}...", AGENT_PATH);

    register_agent(&connection, &agent, "DisplayOnly")
        .await?;

    println!("Agent registered!");
    
    println!("Creating advertisement...");

    let advert = hid::GamePadAdvertisement::new(1);

    println!("Registering advertisement...");

    register_advertisement(&connection, &advert.get_path())
        .await?;

    println!("Advertisement registered!");

    loop {}
}