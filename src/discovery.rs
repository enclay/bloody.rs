use rusb::{GlobalContext, Device};
use crate::opcode;


type Devices = Vec<Device<GlobalContext>>;

pub fn bloody_devices() -> Devices {

    rusb::devices().unwrap().iter().filter(|dev| {
        let desc = dev.device_descriptor().unwrap();
        desc.vendor_id() == opcode::A4TECH_PRODUCT
    }).collect::<Devices>()

}
