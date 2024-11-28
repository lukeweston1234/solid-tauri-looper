use crate::app::app_controller::AppController;
use std::sync::Arc;
use tauri::command;

#[command]
pub fn play(app_controller: tauri::State<Arc<AppController>>) {
    app_controller.play();
}

#[command]
pub fn pause(app_controller: tauri::State<Arc<AppController>>) {
    app_controller.pause();
}

#[command]
pub fn stop(app_controller: tauri::State<Arc<AppController>>) {
    app_controller.stop();
}

#[command]
pub fn set_mixer_gain(
    app_controller: tauri::State<Arc<AppController>>,
    track_index: usize,
    gain: f32,
) {
    app_controller.set_mixer_gain(track_index, gain);
}

#[command]
pub fn set_mixer_reverb_wet(
    app_controller: tauri::State<Arc<AppController>>,
    track_index: usize,
    wet: f32,
) {
    app_controller.set_mixer_reverb_mix(track_index, wet);
}

#[command]
pub fn set_master_reverb_wet(app_controller: tauri::State<Arc<AppController>>, wet: f32) {
    app_controller.set_master_reverb_wet(wet);
}

#[command]
pub fn set_master_gain(app_controller: tauri::State<Arc<AppController>>, gain: f32) {
    app_controller.set_master_gain(gain);
}

#[command]
pub fn record(app_controller: tauri::State<Arc<AppController>>, track_index: usize) {
    app_controller.record(track_index);
}

#[command]
pub fn toggle_solo(app_controller: tauri::State<Arc<AppController>>, track_index: usize) {
    app_controller.toggle_solo(track_index);
}

#[command]
pub fn toggle_mute(app_controller: tauri::State<Arc<AppController>>, track_index: usize) {
    app_controller.toggle_mute(track_index);
}

#[command]
pub fn track_only_feedback(app_controller: tauri::State<Arc<AppController>>, track_index: usize) {
    app_controller.track_only_feedback(track_index);
}

#[command]
pub fn set_time_information(
    app_controller: tauri::State<Arc<AppController>>,
    bpm: u32,
    beat_value: u32,
    beats_per_measure: u32,
    bars: u32,
) {
    app_controller.set_time_information(bpm, beat_value, beats_per_measure, bars);
}

#[command]
pub fn start_looping(app_controller: tauri::State<Arc<AppController>>) {
    app_controller.advance_looper();
}

#[command]
pub fn start_metronome(app_controller: tauri::State<Arc<AppController>>) {
    println!("Start metronome!");
    app_controller.start_metronome();
}

#[command]
pub fn stop_metronome(app_controller: tauri::State<Arc<AppController>>) {
    app_controller.stop_metronome();
}

#[command]
pub fn reset(app_controller: tauri::State<Arc<AppController>>) {
    app_controller.reset();
}
