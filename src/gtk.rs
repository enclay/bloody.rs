
extern crate rusb;

use glib::clone;
use gtk::prelude::*;
use gtk::RadioMenuItem;
use libappindicator::{AppIndicator, AppIndicatorStatus};

use crate::mouse::Mouse;

use std::cell::RefCell;
use std::rc::Rc;
use std::env;
use std::path::Path;

fn create_level_item(previous: Option<&RadioMenuItem>, m: &Rc<RefCell<Mouse>>, level: u8) -> RadioMenuItem {

    let item = match previous {
        Some(pr) => RadioMenuItem::from_widget(pr),
        None => RadioMenuItem::new()
    };

    item.set_label(format!("Level {}", level).as_str());

    item.connect_activate(clone!(@strong m => move |_| {
        m.borrow_mut().set_backlight(level);
    }));

    item
}

fn create_tray_menu(mouse: Rc<RefCell<Mouse>>) -> gtk::Menu {
    let menu = gtk::Menu::new();
    
    let item0 = &create_level_item(None, &mouse, 0);    
    let item1 = &create_level_item(Some(item0), &mouse, 1);    
    let item2 = &create_level_item(Some(item1), &mouse, 2);
    let item3 = &create_level_item(Some(item2), &mouse, 3);

    menu.append(item0);
    menu.append(item1);
    menu.append(item2);
    menu.append(item3);


    menu.append(&gtk::SeparatorMenuItem::new());
    menu.append(&gtk::MenuItem::with_label("Show Settings"));

    menu
}
pub fn main(mouse: Mouse) {

    gtk::init().unwrap();

    let mut indicator = AppIndicator::new("bloody tray widget", "");

    let icon_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
    indicator.set_icon_theme_path(icon_path.to_str().unwrap());
    indicator.set_icon_full("white_mouse", "icon");

    indicator.set_status(AppIndicatorStatus::Active);

    let mut menu = create_tray_menu(Rc::new(RefCell::new(mouse)));

    indicator.set_menu(&mut menu);
    menu.show_all();

    gtk::main();
}
