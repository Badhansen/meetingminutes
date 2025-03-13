// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::process::Command;
use std::env;

fn start_backend() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    println!("Current directory: {:?}", current_dir);
    
    let backend_path = current_dir.parent().unwrap_or(&current_dir).join("backend").join("main.py");
    println!("Looking for backend at: {:?}", backend_path);

    let _backend_process = Command::new("python3")
        .arg("../backend/main.py")
        .spawn()
        .expect("Failed to start FastAPI backend");
}

fn main() {
  start_backend();
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
