use crate::app::app_controller::AppController;
use tauri::command;

#[command]
pub fn play(app_controller: tauri::State<AppController>) {
    app_controller.play();
}

#[command]
pub fn pause(app_controller: tauri::State<AppController>) {
    app_controller.pause();
}

#[command]
pub fn stop(app_controller: tauri::State<AppController>) {
    app_controller.stop();
}

#[command]
pub fn set_mixer_gain(app_controller: tauri::State<AppController>, track_index: usize, gain: f32) {
    app_controller.set_mixer_gain(track_index, gain);
}

#[command]
pub fn set_mixer_reverb_mix(
    app_controller: tauri::State<AppController>,
    track_index: usize,
    mix: f32,
) {
    app_controller.set_mixer_reverb_mix(track_index, mix);
}

#[command]
pub fn record(app_controller: tauri::State<AppController>, track_index: usize) {
    app_controller.record(track_index);
}

#[command]
pub fn track_only_feedback(app_controller: tauri::State<AppController>, track_index: usize) {
    app_controller.track_only_feedback(track_index);
}

#[command]
pub fn start_looping(app_controller: tauri::State<AppController>) {
    app_controller.advance_looper();
}

#[command]
pub fn start_metronome(app_controller: tauri::State<AppController>) {
    println!("Start metronome!");
    app_controller.start_metronome();
}

#[command]
pub fn stop_metronome(app_controller: tauri::State<AppController>) {
    app_controller.stop_metronome();
}

#[command]
pub fn reset(app_controller: tauri::State<AppController>) {
    app_controller.reset();
}
