use app::system_info::emit_system_info;
use tauri::Manager;

use crate::api::api::*;
use crate::app::app_controller::run_app;
use crate::app::runtime::build_runtime;

mod api;
mod app;
mod audio;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (app_controller, runtime) = build_runtime();

    tauri::Builder::default()
        .manage(app_controller)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            play,
            pause,
            stop,
            record,
            reset,
            start_looping,
            track_only_feedback,
            set_mixer_gain,
            set_mixer_reverb_mix,
            start_metronome,
            stop_metronome
        ])
        .setup(|app| {
            let app_handle = app.app_handle();

            run_app(runtime, app_handle.clone());

            emit_system_info(app_handle.clone());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
