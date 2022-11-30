use crate::rusb::{GlobalContext, Device};
use crate::opcode;

pub fn bloody_devices() -> Vec<Device<GlobalContext>> {

    let mut devices = Vec::new();

    rusb::devices().unwrap().iter().for_each(|dev| {
        let desc = dev.device_descriptor().unwrap();

        if desc.vendor_id() == opcode::A4TECH_PRODUCT {
            devices.push(dev);
        }
    });
    
    devices
}
