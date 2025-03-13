use crate::bluez::register_advertisement;
use crate::bluez::register_agent;
use crate::constants::{ADVERT_PATH, AGENT_PATH};
use zbus::{Connection, Result};

mod bluez;
mod constants;
mod gamepad_values;
mod hid;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting GamepadKI...");

    let connection = Connection::system().await?;

    println!("Connection established!");

    let agent = bluez::Agent::new(AGENT_PATH.to_string());

    connection
        .object_server()
        .at(AGENT_PATH.to_string(), agent)
        .await?;

    println!("Registering agent with path {}...", AGENT_PATH);

    register_agent(&connection, AGENT_PATH, "DisplayOnly").await?;

    println!("Agent registered!");

    println!("Creating advertisement...");

    let advert = hid::create_advertisement(ADVERT_PATH);

    connection.object_server().at(ADVERT_PATH, advert).await?;

    println!("Registering advertisement...");

    register_advertisement(&connection, ADVERT_PATH.to_string()).await?;

    println!("Advertisement registered!");

    loop {}
}
