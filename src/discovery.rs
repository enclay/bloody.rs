use crate::rusb::{GlobalContext, Device};
use crate::opcode;

pub fn bloody_devices() -> Vec<Device<GlobalContext>> {

    let mut devs = Vec::new();

    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        if device_desc.vendor_id() == opcode::A4TECH_PRODUCT {
            devs.push(device);
        }
    }

    devs
}
