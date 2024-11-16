use crate::audio::track::TrackController;
use crate::audio::{metronome::MetronomeController, mixer::MixerNode};
use crossbeam_channel::{bounded, Receiver, Sender};
use crossbeam_channel::{select, select_biased};
use fundsp::hacker32::*;
use std::sync::Arc;
use std::usize;
use tauri::{AppHandle, Emitter};

#[derive(Clone)]
pub enum AppControllerEnum {
    Play,
    Pause,
    Stop,
    Reset,
    AdvanceLooper, // change to reference
    Record(usize),
    TrackOnlyFeedback(usize),
    Exit,
    SetMixerGain(usize, f32),
    SetMixerReverbMix(usize, f32),
    SetMasterGain(f32),
    SetMasterReverbMix(f32),
    SetBPM(u32),
    SetBeatsPerMeasure(u32),
    SetBeatValue(u32),
    SetBars(u32),
    ToggleMute(usize),
    ToggleSolo(usize),
    StartMetronome,
    StopMetronome,
    SendAddTrackEvent,
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
    MixerFeedback(An<MixerNode<7>>),
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
            MixerNodeEnum::MixerFeedback(_) => (),
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
            MixerNodeEnum::MixerFeedback(_) => (),
        }
    }

    fn toggle_mute(&mut self) {
        match self {
            MixerNodeEnum::MixerOne(node) => node.toggle_mute(),
            MixerNodeEnum::MixerTwo(node) => node.toggle_mute(),
            MixerNodeEnum::MixerThree(node) => node.toggle_mute(),
            MixerNodeEnum::MixerFour(node) => node.toggle_mute(),
            MixerNodeEnum::MixerFive(node) => node.toggle_mute(),
            MixerNodeEnum::MixerSix(node) => node.toggle_mute(),
            MixerNodeEnum::MixerFeedback(_) => (),
        }
    }

    fn mute(&mut self) {
        match self {
            MixerNodeEnum::MixerOne(node) => node.mute(),
            MixerNodeEnum::MixerTwo(node) => node.mute(),
            MixerNodeEnum::MixerThree(node) => node.mute(),
            MixerNodeEnum::MixerFour(node) => node.mute(),
            MixerNodeEnum::MixerFive(node) => node.mute(),
            MixerNodeEnum::MixerSix(node) => node.mute(),
            MixerNodeEnum::MixerFeedback(_) => (),
        }
    }

    fn unmute(&mut self) {
        match self {
            MixerNodeEnum::MixerOne(node) => node.unmute(),
            MixerNodeEnum::MixerTwo(node) => node.unmute(),
            MixerNodeEnum::MixerThree(node) => node.unmute(),
            MixerNodeEnum::MixerFour(node) => node.unmute(),
            MixerNodeEnum::MixerFive(node) => node.unmute(),
            MixerNodeEnum::MixerSix(node) => node.unmute(),
            MixerNodeEnum::MixerFeedback(_) => (),
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
    pub fn start_metronome(&self) {
        let _ = self.sender.send(AppControllerEnum::StartMetronome);
    }
    pub fn stop_metronome(&self) {
        let _ = self.sender.send(AppControllerEnum::StopMetronome);
    }
    pub fn set_bars(&self, bars: u32) {
        let _ = self.sender.send(AppControllerEnum::SetBars(bars));
    }
    pub fn set_bpm(&self, bpm: u32) {
        let _ = self.sender.send(AppControllerEnum::SetBPM(bpm));
    }
    pub fn set_beat_value(&self, beat_value: u32) {
        let _ = self
            .sender
            .send(AppControllerEnum::SetBeatValue(beat_value));
    }
    pub fn reset(&self) {
        let _ = self.sender.send(AppControllerEnum::Reset);
    }
    pub fn set_master_reverb_wet(&self, wet: f32) {
        let _ = self.sender.send(AppControllerEnum::SetMasterReverbMix(wet));
    }
    pub fn set_master_gain(&self, gain: f32) {
        let _ = self.sender.send(AppControllerEnum::SetMasterGain(gain));
    }
    pub fn toggle_mute(&self, index: usize) {
        let _ = self.sender.send(AppControllerEnum::ToggleMute(index));
    }
    pub fn toggle_solo(&self, index: usize) {
        let _ = self.sender.send(AppControllerEnum::ToggleSolo(index));
    }
    pub fn add_track_to_client(&self){
        let _ = self.sender.send(AppControllerEnum::SendAddTrackEvent);
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
    metronome_controller: Arc<MetronomeController>,
    master_gain: Shared,
    master_reverb_wet: Shared,
    is_solo: bool,
}
impl App {
    pub fn new(
        app_controller_receiver: Receiver<AppControllerEnum>,
        next_loop_receiver: Receiver<()>,
        audio_visualization_receiver: Receiver<f32>,
        mixers: Vec<MixerNodeEnum>,
        track_controllers: Vec<TrackController>,
        metronome_controller: Arc<MetronomeController>,
        master_gain: Shared,
        master_reverb_wet: Shared,
    ) -> Self {
        Self {
            bpm: 120,
            bars: 4,
            beat_value: 4,
            beats_per_measure: 4,
            app_controller_receiver,
            active_recording_track_index: None,
            track_size: 6, // 7th track is feedback only
            next_loop_receiver,
            audio_visualization_receiver,
            state: AppControllerEnum::Stop,
            track_controllers,
            mixers,
            metronome_controller: metronome_controller,
            master_gain,
            master_reverb_wet,
            is_solo: false,
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
    pub fn reset(&mut self) {
        for track in self.track_controllers.iter_mut() {
            track.clear_sample();
            track.stop();
            self.active_recording_track_index = Some(0);
        }
        self.track_only_feedback(0);
    }
    pub fn advance_looping_track(&mut self, app_handle: &AppHandle) {
        if let Some(track_index) = self.active_recording_track_index {
            println!("{:?}", track_index);
            if  track_index < self.track_size {
                self.active_recording_track_index = Some(track_index + 1);
                self.record(track_index + 1);
                let _ = app_handle.emit("track_added", self.active_recording_track_index);
            } else {
                self.track_only_feedback(track_index);
            }
        } else {
            self.active_recording_track_index = Some(0);
            self.record(0);
            let _ = app_handle.emit("track_added", self.active_recording_track_index);
        }
    }
    pub fn add_track_to_client(&mut self, app_handle: &AppHandle){
        // used for inti
        let _ = app_handle.emit("track_added", 0);
        self.active_recording_track_index = Some(0);
    }
    pub fn start_metronome(&self) {
        self.metronome_controller.start();
    }
    pub fn stop_metronome(&self) {
        self.metronome_controller.stop();
    }
    pub fn set_master_gain(&self, value: f32) {
        self.master_gain.set(value);
    }
    pub fn set_master_reverb_wet(&self, value: f32) {
        self.master_reverb_wet.set(value);
    }
    pub fn toggle_mute(&mut self, index: usize) {
        if let Some(mixer) = self.mixers.get_mut(index) {
            mixer.toggle_mute();
        }
    }
    pub fn toggle_solo(&mut self, index: usize) {
        if self.is_solo {
            self.unsolo();
            self.is_solo = false;
            return;
        }
        self.solo(index);
        self.is_solo = true;
    }
    pub fn solo(&mut self, index: usize) {
        for (i, mixer) in self.mixers.iter_mut().enumerate() {
            if i != index {
                mixer.mute();
            }
        }
    }
    pub fn unsolo(&mut self) {
        for (i, mixer) in self.mixers.iter_mut().enumerate() {
            mixer.unmute();
        }
    }
}

pub fn build_app(
    mixers: Vec<MixerNodeEnum>,
    track_controllers: Vec<TrackController>,
    next_looper_receiver: Receiver<()>,
    audio_visualization_receiver: Receiver<f32>,
    metronome_controller: Arc<MetronomeController>,
    master_gain: Shared,
    master_reverb_wet: Shared,
) -> (AppController, App) {
    let (sender, receiver) = bounded(10);

    let app_controller = AppController::new(sender);

    let app = App::new(
        receiver,
        next_looper_receiver,
        audio_visualization_receiver,
        mixers,
        track_controllers,
        metronome_controller,
        master_gain,
        master_reverb_wet,
    );

    (app_controller, app)
}

pub fn run_app(mut app: App, app_handle: AppHandle) {
    std::thread::spawn(move || loop {
        select_biased! {
            recv(app.next_loop_receiver) -> msg => {
                match msg {
                    Ok(()) => {
                        app.advance_looping_track(&app_handle);
                    }
                    Err(err) => {
                        eprintln!("Error receiving from next_loop_receiver: {:?}", err);
                    }
                }
            }
            recv(app.app_controller_receiver) -> msg => {
                match msg {
                    Ok(msg) => {
                        app.set_app_state(&msg);
                        match msg {
                            AppControllerEnum::Play => app.play(),
                            AppControllerEnum::Pause => app.pause(),
                            AppControllerEnum::Record(track_index) => app.record(track_index),
                            AppControllerEnum::TrackOnlyFeedback(track_index) => {
                                app.track_only_feedback(track_index)
                            }
                            AppControllerEnum::Stop => app.stop(),
                            AppControllerEnum::Reset => app.reset(),
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
                            AppControllerEnum::StartMetronome => app.start_metronome(),
                            AppControllerEnum::StopMetronome => app.stop_metronome(),
                            AppControllerEnum::SetMasterGain(gain) => app.set_master_gain(gain),
                            AppControllerEnum::SetMasterReverbMix(wet) => app.set_master_reverb_wet(wet),
                            AppControllerEnum::ToggleMute(index) => app.toggle_mute(index),
                            AppControllerEnum::ToggleSolo(index) => app.toggle_solo(index),
                            AppControllerEnum::SendAddTrackEvent => app.add_track_to_client(&app_handle),
                            AppControllerEnum::Exit => break,
                        }
                    }
                    Err(err) => {
                        eprintln!("Error receiving from app_controller_receiver: {:?}", err);
                    }
                }
            }
            recv(app.audio_visualization_receiver) -> msg => {
                match msg {
                    Ok(sample) => {
                        let _ = app_handle.emit("visualizer_sample", sample);
                    }
                    Err(err) => {
                        eprintln!("Error receiving from audio_visualization_receiver: {:?}", err);
                    }
                }
            }
        }
    });
}
