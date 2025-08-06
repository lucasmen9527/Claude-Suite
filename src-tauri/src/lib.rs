// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// Declare modules
pub mod checkpoint;
#[cfg(target_os = "windows")]
pub mod claude_binary;
#[cfg(not(target_os = "windows"))]
pub mod claude_binary_unix;
pub mod claude_binary_common;
pub mod commands;
pub mod process;
pub mod i18n;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
