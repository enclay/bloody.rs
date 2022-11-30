extern crate rusb;

use crate::mouse::Mouse;

mod mouse;
mod io;
mod request_type;
mod discovery;
mod opcode;

fn main() {
    for device in discovery::bloody_devices() {

        let description = device.device_descriptor().unwrap();
        println!("Found compatible device: {}", description.product_id());

        let handle = device.open().unwrap();
        let mouse = Mouse::new(handle);
        mouse.set_backlight(2);
        println!("Current backlight sensivity = {}", mouse.get_backlight());
    }
}
