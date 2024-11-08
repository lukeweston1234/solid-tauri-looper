use std::usize;

use crate::audio::mixer::MixerNode;
use crate::audio::track::TrackController;
use crossbeam_channel::{bounded, Receiver, Sender};
use fundsp::hacker32::*;

#[derive(Clone, Copy)]
pub enum AppControllerEnum {
    Play,
    Pause,
    Stop,
    Loop,
    Record(usize),
    TrackOnlyFeedback(usize),
    Exit,
    SetMixerGain(usize, f32),
    SetMixerReverbMix(usize, f32),
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
}

pub struct App {
    receiver: Receiver<AppControllerEnum>,
    state: AppControllerEnum,
    track_controllers: Vec<TrackController>,
    mixers: Vec<MixerNodeEnum>,
}
impl App {
    pub fn new(
        receiver: Receiver<AppControllerEnum>,
        mixers: Vec<MixerNodeEnum>,
        track_controllers: Vec<TrackController>,
    ) -> Self {
        Self {
            receiver,
            state: AppControllerEnum::Stop,
            track_controllers,
            mixers,
        }
    }
    pub fn set_app_state(&mut self, new_state: AppControllerEnum) {
        self.state = new_state;
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
}

pub fn build_app(
    mixers: Vec<MixerNodeEnum>,
    track_controllers: Vec<TrackController>,
) -> (AppController, App) {
    let (sender, receiver) = bounded(10);

    let app_controller = AppController::new(sender);

    let app = App::new(receiver, mixers, track_controllers);

    (app_controller, app)
}

pub fn run_app(mut app: App) {
    std::thread::spawn(move || loop {
        if let Ok(msg) = app.receiver.recv() {
            app.set_app_state(msg);
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
                AppControllerEnum::Loop => {}
                AppControllerEnum::Exit => break,
            }
        }
    });
}
