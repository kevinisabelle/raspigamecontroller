// rust
use crate::bluez::Advertisement;

// For the gamepad appearance constant, define a constant value. Adjust as needed.
const ADV_APPEARANCE_GAMEPAD: u16 = 0x03c0;

pub fn create_advertisement(path: &str) -> Advertisement
{
    let mut adv = Advertisement::new(path, "peripheral".to_string());
    adv.add_service_uuid("1812".to_string());
    adv.add_local_name("KiGP".to_string());
    // Set the appearance and include_tx_power fields
    adv.set_appearance(ADV_APPEARANCE_GAMEPAD);
    adv.set_include_tx_power(true);
    adv
}