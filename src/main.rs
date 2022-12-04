extern crate rusb;

use gtk::prelude::*;
use libappindicator::{AppIndicator, AppIndicatorStatus};
use std::cell::RefCell;
use std::rc::Rc;
use glib::clone;
use crate::mouse::Mouse;

mod mouse;
mod reqtype;
mod discovery;
mod opcode;

fn create_level_item(mouse: &Rc<RefCell<Mouse>>, level: u8) -> gtk::MenuItem {
    let item = gtk::MenuItem::with_label(format!("level {}", level).as_str());

    item.connect_activate(clone!(@strong mouse => move |_| {
        mouse.borrow_mut().set_backlight(level);
    }));

    return item;
}

fn init_mouse() -> Mouse {

    let firstdevice = discovery::bloody_devices();
    let device = firstdevice.first().unwrap();
    
    let description = device.device_descriptor().unwrap();
    println!("Found compatible device: {}", description.product_id());

    let handle = device.open().unwrap();
    let mut m = Mouse::new(handle);
    m.detach_driver_if_needed();
    m.claim();
    return m;
}

fn create_tray_menu(mouse: Rc<RefCell<Mouse>>) -> gtk::Menu {
    let menu = gtk::Menu::new();

    menu.append(&create_level_item(&mouse, 0));
    menu.append(&create_level_item(&mouse, 1));
    menu.append(&create_level_item(&mouse, 2));
    menu.append(&create_level_item(&mouse, 3));

    return menu;
}

fn main() {
    gtk::init().unwrap();

    let mut indicator = AppIndicator::new("bloody tray widget", "");
    indicator.set_status(AppIndicatorStatus::Active);

    let mouse = Rc::new(RefCell::new(init_mouse()));
    let mut menu = create_tray_menu(mouse);

    indicator.set_menu(&mut menu);
    menu.show_all();

    gtk::main();
}
