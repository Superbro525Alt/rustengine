// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod engine;
mod example;

// use crate::engine::
#[cfg(windows)]
extern crate winapi;

fn main() {
#[cfg(target_os = "windows")]
  unsafe {
    winapi::um::shellscalingapi::SetProcessDpiAwareness(1);
  }

    example::main();

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
