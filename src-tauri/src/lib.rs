use crate::api::api::*;
use crate::app::runtime::build_runtime;

mod api;
mod app;
mod audio;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_controller = build_runtime();

    app_controller.track_only_feedback(0);

    tauri::Builder::default()
        .manage(app_controller)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            play,
            pause,
            stop,
            record,
            start_looping,
            track_only_feedback,
            set_mixer_gain,
            set_mixer_reverb_mix,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
