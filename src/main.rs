extern crate rusb;

use crate::mouse::Mouse;

mod mouse;
mod io;
mod request_type;
mod opcode;

fn main() {

    for device in rusb::devices().unwrap().iter() {
        let device_desc = device.device_descriptor().unwrap();

        if device_desc.vendor_id() == opcode::A4TECH_PRODUCT {
            println!("Found a4tech device: {}", device_desc.product_id());

            let handle = device.open().unwrap();
            
            let mouse = Mouse::new(handle);
            mouse.set_backlight(0);
        }
    }

}
