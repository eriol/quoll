use std::ffi::CStr;
use std::process;
use std::sync::{Arc, RwLock};

use gtk::{MenuItemExt, MenuShellExt, WidgetExt};
use libappindicator::{AppIndicator, AppIndicatorStatus};
use log::error;

use crate::commands::Command;

const REFRESH_INTERVAL: u32 = 500;

fn create_indicator() -> AppIndicator {
    let mut indicator = AppIndicator::new("quoll", "");
    indicator.set_status(AppIndicatorStatus::APP_INDICATOR_STATUS_ACTIVE);

    indicator
}

fn create_menu() -> gtk::Menu {
    let menu = gtk::Menu::new();
    let quit: &CStr = unsafe { CStr::from_ptr(gtk_sys::GTK_STOCK_QUIT) };
    if let Ok(quit) = quit.to_str() {
        let menu_quit = gtk::ImageMenuItem::new_from_stock(quit, None);
        menu_quit.connect_activate(|_| {
            gtk::main_quit();
        });
        menu.append(&menu_quit);
    }

    menu
}

fn build_ui(command: Arc<RwLock<Command>>) {
    let mut indicator = create_indicator();
    let mut menu = create_menu();
    indicator.set_menu(&mut menu);
    menu.show_all();

    gtk::timeout_add(REFRESH_INTERVAL, move || {
        if let Ok(command) = command.read() {
            match *command {
                Command::Quit => gtk::main_quit(),
                Command::Custom(_) => {
                    if let Some(path) = command.to_path() {
                        if let Some(path) = path.to_str() {
                            indicator.set_icon_full(path, "icon");
                        }
                    }
                }
            }
        }
        gtk::Continue(true)
    });
}

pub fn start(command: Arc<RwLock<Command>>) {
    if gtk::init().is_err() {
        error!("GTK initialization error.");
        process::exit(1);
    }

    build_ui(command);

    gtk::main();
}
