use crate::audio::mixer::MixerNode;
use crate::audio::track::TrackController;
use crossbeam_channel::{bounded, Receiver, Sender};
use fundsp::hacker32::*;
use std::{clone, usize};
use tauri::{AppHandle, Emitter};

#[derive(Clone)]
pub enum AppControllerEnum {
    Play,
    Pause,
    Stop,
    AdvanceLooper, // change to reference
    Record(usize),
    TrackOnlyFeedback(usize),
    Exit,
    SetMixerGain(usize, f32),
    SetMixerReverbMix(usize, f32),
    SetBPM(u32),
    SetBeatsPerMeasure(u32),
    SetBeatValue(u32),
    SetBars(u32),
}

// This is gross, but I want to access nodes from an index,
// and I am having a hard time boxing the constant mixers from FunDSP
pub enum MixerNodeEnum {
    MixerOne(An<MixerNode<1>>),
    MixerTwo(An<MixerNode<2>>),
    MixerThree(An<MixerNode<3>>),
    MixerFour(An<MixerNode<4>>),
    MixerFive(An<MixerNode<5>>),
    MixerSix(An<MixerNode<6>>),
}
impl MixerNodeEnum {
    fn set_gain(&self, gain: f32) {
        match self {
            MixerNodeEnum::MixerOne(node) => node.set_gain(gain),
            MixerNodeEnum::MixerTwo(node) => node.set_gain(gain),
            MixerNodeEnum::MixerThree(node) => node.set_gain(gain),
            MixerNodeEnum::MixerFour(node) => node.set_gain(gain),
            MixerNodeEnum::MixerFive(node) => node.set_gain(gain),
            MixerNodeEnum::MixerSix(node) => node.set_gain(gain),
        }
    }

    fn set_reverb_mix(&self, mix: f32) {
        match self {
            MixerNodeEnum::MixerOne(node) => node.set_reverb_mix(mix),
            MixerNodeEnum::MixerTwo(node) => node.set_reverb_mix(mix),
            MixerNodeEnum::MixerThree(node) => node.set_reverb_mix(mix),
            MixerNodeEnum::MixerFour(node) => node.set_reverb_mix(mix),
            MixerNodeEnum::MixerFive(node) => node.set_reverb_mix(mix),
            MixerNodeEnum::MixerSix(node) => node.set_reverb_mix(mix),
        }
    }
}
pub struct AppController {
    sender: Sender<AppControllerEnum>,
}
impl AppController {
    pub fn new(sender: Sender<AppControllerEnum>) -> Self {
        Self { sender }
    }
    pub fn play(&self) {
        let _ = self.sender.send(AppControllerEnum::Play).unwrap();
    }
    pub fn pause(&self) {
        let _ = self.sender.send(AppControllerEnum::Pause).unwrap();
    }
    pub fn stop(&self) {
        let _ = self.sender.send(AppControllerEnum::Stop).unwrap();
    }
    pub fn set_mixer_gain(&self, track_index: usize, gain: f32) {
        let _ = self
            .sender
            .send(AppControllerEnum::SetMixerGain(track_index, gain));
    }
    pub fn set_mixer_reverb_mix(&self, track_index: usize, mix: f32) {
        let _ = self
            .sender
            .send(AppControllerEnum::SetMixerReverbMix(track_index, mix));
    }
    pub fn record(&self, track_index: usize) {
        let _ = self.sender.send(AppControllerEnum::Record(track_index));
    }
    pub fn track_only_feedback(&self, track_index: usize) {
        let _ = self
            .sender
            .send(AppControllerEnum::TrackOnlyFeedback(track_index));
    }
    pub fn advance_looper(&self) {
        println!("Advance looper!");
        let _ = self.sender.send(AppControllerEnum::AdvanceLooper);
    }
    pub fn exit(&self) {
        let _ = self.sender.send(AppControllerEnum::Exit);
    }
}

pub struct App {
    bpm: u32,
    bars: u32,
    beat_value: u32,        // 4/(4), 3/(2)
    beats_per_measure: u32, // (4)/4, (6)/8
    app_controller_receiver: Receiver<AppControllerEnum>,
    active_recording_track_index: Option<usize>,
    track_size: usize,
    next_loop_receiver: Receiver<()>,
    audio_visualization_receiver: Receiver<f32>,
    state: AppControllerEnum,
    track_controllers: Vec<TrackController>,
    mixers: Vec<MixerNodeEnum>,
}
impl App {
    pub fn new(
        app_controller_receiver: Receiver<AppControllerEnum>,
        next_loop_receiver: Receiver<()>,
        audio_visualization_receiver: Receiver<f32>,
        mixers: Vec<MixerNodeEnum>,
        track_controllers: Vec<TrackController>,
    ) -> Self {
        Self {
            bpm: 120,
            bars: 4,
            beat_value: 4,
            beats_per_measure: 4,
            app_controller_receiver,
            active_recording_track_index: None,
            track_size: mixers.len(),
            next_loop_receiver,
            audio_visualization_receiver,
            state: AppControllerEnum::Stop,
            track_controllers,
            mixers,
        }
    }
    pub fn set_app_state(&mut self, new_state: &AppControllerEnum) {
        self.state = new_state.clone();
    }
    pub fn play(&self) {
        for track_controller in self.track_controllers.iter() {
            track_controller.play();
        }
    }
    pub fn pause(&self) {
        for track_controller in self.track_controllers.iter() {
            track_controller.pause();
        }
    }
    pub fn stop(&self) {
        for track_controller in self.track_controllers.iter() {
            track_controller.stop();
        }
    }
    pub fn record(&self, track_index: usize) {
        if let Some(track) = self.track_controllers.get(track_index) {
            track.record();
        } else {
            println!("Could not find track at index {}", track_index);
        }
    }
    pub fn set_mixer_gain(&self, track_index: usize, gain: f32) {
        if let Some(mixer) = self.mixers.get(track_index) {
            mixer.set_gain(gain);
        }
    }
    pub fn set_mixer_reverb_mix(&self, track_index: usize, mix: f32) {
        if let Some(mixer) = self.mixers.get(track_index) {
            mixer.set_reverb_mix(mix);
        }
    }
    pub fn track_only_feedback(&self, track_index: usize) {
        if let Some(track) = self.track_controllers.get(track_index) {
            track.only_input();
        }
    }
    pub fn set_bpm(&mut self, bpm: u32) {
        self.bpm = bpm;
    }
    pub fn set_beats_per_measure(&mut self, beats_per_measure: u32) {
        self.beats_per_measure = beats_per_measure;
    }
    pub fn set_bars(&mut self, bars: u32) {
        self.bars = bars;
    }
    pub fn set_beat_value(&mut self, beat_value: u32) {
        self.beat_value = beat_value;
    }
    pub fn advance_looping_track(&mut self, app_handle: &AppHandle) {
        if let Some(track_index) = self.active_recording_track_index {
            println!("{:?}", track_index);
            if self.track_size > track_index {
                self.active_recording_track_index = Some(track_index + 1);
                self.record(track_index + 1);
                let _ = app_handle.emit("track_added", self.active_recording_track_index);
            }
        } else {
            self.active_recording_track_index = Some(0);
            self.record(0);
            let _ = app_handle.emit("track_added", self.active_recording_track_index);
        }
    }
}

pub fn build_app(
    mixers: Vec<MixerNodeEnum>,
    track_controllers: Vec<TrackController>,
    next_looper_receiver: Receiver<()>,
    audio_visualization_receiver: Receiver<f32>,
) -> (AppController, App) {
    let (sender, receiver) = bounded(10);

    let app_controller = AppController::new(sender);

    let app = App::new(
        receiver,
        next_looper_receiver,
        audio_visualization_receiver,
        mixers,
        track_controllers,
    );

    (app_controller, app)
}

pub fn run_app(mut app: App, app_handle: AppHandle) {
    std::thread::spawn(move || loop {
        // TODO, non blocking with a sleep seems better than a tight loop, but this needs to be refactored
        if let Ok(msg) = app.app_controller_receiver.try_recv() {
            app.set_app_state(&msg);
            match msg {
                AppControllerEnum::Play => app.play(),
                AppControllerEnum::Pause => app.pause(),
                AppControllerEnum::Record(track_index) => app.record(track_index),
                AppControllerEnum::TrackOnlyFeedback(track_index) => {
                    app.track_only_feedback(track_index)
                }
                AppControllerEnum::Stop => app.stop(),
                AppControllerEnum::SetMixerGain(track_index, gain) => {
                    app.set_mixer_gain(track_index, gain)
                }
                AppControllerEnum::SetMixerReverbMix(track_index, mix) => {
                    app.set_mixer_reverb_mix(track_index, mix);
                }
                AppControllerEnum::SetBPM(bpm) => app.set_bpm(bpm),
                AppControllerEnum::SetBars(bars) => app.set_bars(bars),
                AppControllerEnum::SetBeatValue(beat_value) => app.set_beat_value(beat_value),
                AppControllerEnum::SetBeatsPerMeasure(beats_per_measure) => {
                    app.set_beats_per_measure(beats_per_measure)
                }
                AppControllerEnum::AdvanceLooper => app.advance_looping_track(&app_handle),
                AppControllerEnum::Exit => break,
            }
        }

        if let Ok(()) = app.next_loop_receiver.try_recv() {
            app.advance_looping_track(&app_handle);
        }

        if let Ok(sample) = app.audio_visualization_receiver.try_recv() {
            let _ = app_handle.emit("visualizer_sample", sample);
        }

        std::thread::sleep(std::time::Duration::from_millis(10));
    });
}
