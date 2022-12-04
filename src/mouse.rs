extern crate rusb;

use crate::io;
use crate::opcode;
use crate::opcode::BACKLIGHT_READ;
use rusb::{DeviceHandle, GlobalContext};

pub struct Mouse {
    _dev: io::Device
}

impl Mouse {
    pub fn new(dev: DeviceHandle<GlobalContext>) -> Mouse {
        Mouse { _dev: io::Device::new(dev) }
    }

    pub fn set_backlight(&self, level: u8) {
        let buf: [u8; 9] = [
            opcode::A4TECH_MAGIC,
            opcode::BACKLIGHT_OPCODE, 0, 0,
            opcode::BACKLIGHT_WRITE, 0, 0, 0,
            level
        ];
        self._dev.write(&buf);
    }

    pub fn get_backlight(&self) -> u8 {
        let buf: [u8; 6] = [
            opcode::A4TECH_MAGIC,
            opcode::BACKLIGHT_OPCODE, 0, 0,
            BACKLIGHT_READ, 0
        ];

        let response = self._dev.read(&buf);
        return response[8];
    }

}
