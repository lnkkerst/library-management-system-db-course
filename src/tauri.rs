// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello {}", name)
}

pub fn run_tauri(url: &str) {
    let url = url.to_string();
    tauri::Builder::default()
        .setup(move |app| {
            app.get_window("main")
                .unwrap()
                .eval(&format!("window.location.replace({url})"))
                .unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
