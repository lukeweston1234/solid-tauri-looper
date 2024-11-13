use super::{playable::Playable, sampler::Sampler};
use crate::audio::audio_sample::load_wav;
use crossbeam_channel::{bounded, Receiver, Sender};
use std::sync::Arc;
use std::{env::current_dir, time::Instant};

#[derive(PartialEq)]
pub enum MetronomeState {
    Start,
    Stop,
}

pub struct MetronomeController {
    sender: Sender<MetronomeState>,
}
impl MetronomeController {
    pub fn new(sender: Sender<MetronomeState>) -> Self {
        Self { sender }
    }
    pub fn start(&self) {
        let _ = self.sender.send(MetronomeState::Start);
    }
    pub fn stop(&self) {
        let _ = self.sender.send(MetronomeState::Stop);
    }
}

pub struct Metronome {
    state: MetronomeState,
    sender: Sender<(f32, f32)>,
    controller_receiver: Receiver<MetronomeState>,
    bpm: u32,
    sampler: Sampler<f32>,
}
impl Metronome {
    pub fn new(
        bpm: u32,
        sender: Sender<(f32, f32)>,
        controller_receiver: Receiver<MetronomeState>,
    ) -> Self {
        println!("ENV:{:?}", current_dir());
        let sample = load_wav("./assets/metronome.wav").expect("Could not load metronome");
        let mut sampler = Sampler::new(Some(sample));
        sampler.set_is_looping(false);
        Self {
            state: MetronomeState::Stop,
            sender,
            controller_receiver,
            bpm: bpm,
            sampler,
        }
    }
    pub fn start(&mut self) {
        println!("In self.start");
        self.state = MetronomeState::Start;
    }
    pub fn stop(&mut self) {
        self.state = MetronomeState::Stop;
    }
}

impl Playable<f32> for Metronome {
    fn next_sample(&mut self) -> Option<(f32, f32)> {
        match self.state {
            MetronomeState::Start => self.sampler.next_sample(),
            MetronomeState::Stop => None,
        }
    }
}

pub fn run_metronome(mut metronome: Metronome) {
    std::thread::spawn(move || {
        let mut last_beat = Instant::now();
        loop {
            if let Ok(msg) = metronome.controller_receiver.try_recv() {
                match msg {
                    MetronomeState::Start => metronome.start(),
                    MetronomeState::Stop => metronome.stop(),
                }
            }
            if metronome.state == MetronomeState::Start {
                let interval = std::time::Duration::from_secs_f32(60.0 / metronome.bpm as f32);

                if last_beat.elapsed() >= interval {
                    last_beat = Instant::now();
                    metronome.sampler.play();
                }

                if let Some(sample) = metronome.sampler.next_sample() {
                    let _ = metronome.sender.send(sample);
                }
            } else {
                std::thread::sleep(std::time::Duration::from_millis(3));
            }
        }
    });
}

pub fn build_metronome(bpm: u32) -> (Arc<MetronomeController>, Metronome, Receiver<(f32, f32)>) {
    let (controller_sender, controller_receiver) = bounded(10);

    let (metronome_buffer_sender, metronome_buffer_receiver) = bounded(512);

    let metronome_controller = MetronomeController::new(controller_sender);

    let metronome = Metronome::new(bpm, metronome_buffer_sender, controller_receiver);

    (
        Arc::new(metronome_controller),
        metronome,
        metronome_buffer_receiver,
    )
}
