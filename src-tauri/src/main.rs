// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(async_closure)]

use std::sync::{Arc, Mutex};

use futures::executor::block_on;

use lazy_static::lazy_static;
use log::info;
// use pollster::{block_on, FutureExt};
use tokio::runtime::Runtime;
use winit::event_loop::EventLoop;

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

mod engine;
mod example;

// use crate::engine::
#[cfg(windows)]
extern crate winapi;

async fn get_engine(data: String) {
    let (eng, l) = engine::state::Engine::import_from_json(data, None).await;

    engine::state::Engine::run(Arc::new(Mutex::new(eng)), l);
}

#[tauri::command]
async fn start_preview(data: String) {
    block_on(get_engine(data));

    info!("Done");
    // block_on(e);
    // print!("{}", e);
}

fn main() {
    #[cfg(target_os = "windows")]
    unsafe {
        winapi::um::shellscalingapi::SetProcessDpiAwareness(1);
    }

    example::init();

    let save = CustomMenuItem::new("save".to_string(), "Save");
    let exit = CustomMenuItem::new("exit".to_string(), "Exit");
    let back = CustomMenuItem::new("back".to_string(), "Back");
    let load = CustomMenuItem::new("load".to_string(), "Load");
    let undo = CustomMenuItem::new("undo".to_string(), "Undo").disabled();
    let redo = CustomMenuItem::new("redo".to_string(), "Redo").disabled();

    let options_menu = Menu::new()
        .add_item(save)
        .add_item(load)
        .add_native_item(MenuItem::Separator)
        .add_item(back)
        .add_item(exit)
        .add_native_item(MenuItem::Separator)
        .add_item(undo)
        .add_item(redo);

    let menu = Menu::new()
        .add_submenu(Submenu::new("File", options_menu));

    tauri::Builder::default()
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
        "save" => {
            println!("Save menu item clicked");
            // Handle save action
        }
        "exit" => {
            std::process::exit(0);
        }
        "back" => {
            println!("Back menu item clicked");
            // Handle back action
        }
        "load" => {
            println!("Load menu item clicked");
            // Handle load action
        }
        "undo" => {
            println!("Undo menu item clicked (disabled)");
            // Handle undo action (if needed in future)
        }
        "redo" => {
            println!("Redo menu item clicked (disabled)");
            // Handle redo action (if needed in future)
        }
        _ => {}
    })
    .invoke_handler(tauri::generate_handler![start_preview])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
