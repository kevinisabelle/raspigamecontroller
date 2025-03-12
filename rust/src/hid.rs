// rust
use crate::bluez::Advertisement;

// For the gamepad appearance constant, define a constant value. Adjust as needed.
const ADV_APPEARANCE_GAMEPAD: u16 = 0x03c0;

pub struct GamePadAdvertisement {
    pub advertisement: Advertisement,
}

impl GamePadAdvertisement {
    pub fn new(index: u32) -> Self {
        let mut adv = Advertisement::new(index, "peripheral".to_string());
        adv.add_service_uuid("1812".to_string());
        adv.add_local_name("KiGP".to_string());
        // Set the appearance and include_tx_power fields
        adv.set_appearance(ADV_APPEARANCE_GAMEPAD);
        adv.set_include_tx_power(true);
        Self { advertisement: adv }
    }
    
    pub fn get_path(&self) -> zbus::zvariant::ObjectPath {
        self.advertisement.get_path()
    }
}