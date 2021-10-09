use std::process;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use gtk::glib;
use gtk::prelude::*;
use libayatana_appindicator::{AppIndicator, AppIndicatorStatus};
use log::error;

use crate::commands::Command;

const REFRESH_INTERVAL: Duration = Duration::from_millis(500);

fn create_indicator() -> AppIndicator {
    let mut indicator = AppIndicator::new("quoll", "");
    indicator.set_status(AppIndicatorStatus::Active);

    indicator
}

fn create_menu(port: &str) -> gtk::Menu {
    let menu = gtk::Menu::new();

    let port = gtk::MenuItem::with_label(&format!("UDP port: {}", port));

    let quit_item = gtk::MenuItem::new();
    let quit_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let quit_image = gtk::Image::from_icon_name(
        Some("application-exit"),
        gtk::IconSize::Menu,
    );
    let quit_label = gtk::Label::new(Some("Quit"));
    quit_box.pack_start(&quit_image, false, false, 0);
    quit_box.pack_start(&quit_label, true, true, 0);
    quit_item.add(&quit_box);
    quit_item.connect_activate(|_| {
        gtk::main_quit();
    });

    menu.append(&port);
    menu.append(&quit_item);

    menu
}

fn build_ui(command: Arc<RwLock<Command>>, port: &str) {
    let mut indicator = create_indicator();
    let mut menu = create_menu(port);
    indicator.set_menu(&mut menu);
    menu.show_all();

    let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    glib::timeout_add(REFRESH_INTERVAL, move || {
        if let Ok(command) = command.read() {
            match *command {
                Command::Quit => gtk::main_quit(),
                Command::Custom(_) => {
                    if let Some(path) = command.to_path() {
                        let _ = tx.send(path.clone());
                    }
                }
            }
        }
        Continue(true)
    });

    rx.attach(None, move |path| {
        if let Some(path) = path.to_str() {
            indicator.set_icon_full(path, "icon");
        }
        Continue(true)
    });
}

pub fn start(command: Arc<RwLock<Command>>, port: &str) {
    if gtk::init().is_err() {
        error!("GTK initialization error.");
        process::exit(1);
    }

    build_ui(command, port);

    gtk::main();
}
