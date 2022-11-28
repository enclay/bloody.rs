extern crate rusb;

use crate::request_type::{calc_request_type, Direction, Type, Recipient};
use rusb::{DeviceHandle, GlobalContext};
use std::time::Duration;


pub struct Device {
   _handle: DeviceHandle<GlobalContext>
}

const WRITE_TYPE: u8 = calc_request_type(Direction::Out, Type::Class, Recipient::Interface);
//const  READ_TYPE: u8 = calc_request_type(Direction::In,  Type::Class, Recipient::Interface);

impl Device {
    pub fn new(dev: DeviceHandle<GlobalContext>) -> Device {
        Device{ _handle: dev }
    }

    pub fn write(&self, buffer: &[u8]) {

        let res = self._handle.write_control(WRITE_TYPE, 9, 0x0307, 2, &buffer, Duration::new(10, 0));

        match res {
            Ok(_) => {  },
            Err(_) => println!("Write: error")
        }
    }
}
