#![allow(dead_code)]

pub enum Direction {
    Out = 0,
    In = 1,
}

pub enum Type {
    Standard = 0,
    Class = 1,
    Vendor = 2,
    Reserved = 3,
}

pub enum Recipient {
    Device = 0,
    Interface = 1,
    Endpoint = 2,
    Other = 3,
}

pub const fn calc_request_type(direction: Direction, reqtype: Type, recipient: Recipient) -> u8 {
    let d = direction as u8;
    let t = reqtype as u8;
    let r = recipient as u8;

    d << 7 | t << 5 | r
}

