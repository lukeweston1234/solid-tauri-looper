use app::system_info::emit_system_info;
use tauri::{Listener, Manager};
use window_vibrancy::apply_acrylic;

use crate::api::api::*;
use crate::app::app_controller::run_app;
use crate::app::runtime::build_runtime;

mod api;
mod app;
mod audio;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (app_controller, runtime) = build_runtime();

    let app_controller_handle = app_controller.clone();

    tauri::Builder::default()
        .manage(app_controller_handle)
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
            set_mixer_reverb_wet,
            set_master_reverb_wet,
            set_master_gain,
            toggle_solo,
            toggle_mute,
            start_metronome,
            stop_metronome
        ])
        .setup(move |app| {
            let app_handle = app.app_handle();

            let window = app.get_webview_window("main").unwrap();

            apply_acrylic(&window, Some((0, 0, 0, 1))).expect("Unsupported platform");

            run_app(runtime, app_handle.clone());

            app_handle.listen_any("app_ready", move |_event| {
                println!("App Ready");
                app_controller.add_track_to_client();
                app_controller.track_only_feedback(0);
            });

            emit_system_info(app_handle.clone());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
