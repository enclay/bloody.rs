#![allow(unused)]

extern crate rusb;

use rusb::{DeviceHandle, GlobalContext};
use std::time::Duration;

use crate::reqtype::*;
use crate::opcode;

const WRITE_TYPE: u8 = calc_request_type(Direction::Out, Type::Class, Recipient::Interface);
const  READ_TYPE: u8 = calc_request_type(Direction::In,  Type::Class, Recipient::Interface);

pub struct Mouse {
    handle: DeviceHandle<GlobalContext>
}

impl Mouse {
    pub fn new(dev: DeviceHandle<GlobalContext>) -> Mouse {
        Mouse { handle: dev }
    }

    pub fn set_backlight(&self, level: u8) {
        let buf: [u8; 9] = [
            opcode::A4TECH_MAGIC,
            opcode::BACKLIGHT_OPCODE, 0, 0,
            opcode::BACKLIGHT_WRITE, 0, 0, 0,
            level
        ];
        self.write(&buf);
    }

    pub fn get_backlight(&self) -> u8 {
        let buf: [u8; 6] = [
            opcode::A4TECH_MAGIC,
            opcode::BACKLIGHT_OPCODE, 0, 0,
            opcode::BACKLIGHT_READ, 0
        ];

        let response = self.read(&buf);
        return response[8];
    }

    pub fn detach_driver_if_needed(&mut self) {
        if self.is_kernel_driver_active() {
            self.detach_kernel_driver();
            println!("driver successfully detached");
            return;
        }
        println!("driver is already detached");
    }

    pub fn write(&self, buffer: &[u8]) {

        let res = self.handle.write_control(WRITE_TYPE, 9, 0x0307, 2, &buffer, Duration::new(10, 0));
        match res {
            Ok(_) => {  },
            Err(err) => println!("Write: error {}", err )
        }
    }

    pub fn read(&self, buffer: &[u8]) -> Vec<u8> {
        self.write(buffer); 

        let response = &mut[0; 72];

        let res = self.handle.read_control(READ_TYPE, 1, 0x0307, 2, response, Duration::new(10, 0));
        match res {
            Ok(_) => {  },
            Err(_) => println!("Read: error")
        }
        
        response.to_vec()
    }

    pub fn is_kernel_driver_active(&self) -> bool {
        self.handle.kernel_driver_active(2).unwrap()
    }

    pub fn detach_kernel_driver(&mut self) {
        self.handle.detach_kernel_driver(2).unwrap()
    }

    pub fn claim(&mut self) {
        self.handle.claim_interface(2).unwrap()
    }

}
