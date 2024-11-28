use super::{playable::Playable, sampler::Sampler};
use crate::audio::audio_sample::load_wav_from_bytes;
use crossbeam_channel::{bounded, select, tick, Receiver, Sender};
use std::sync::Arc;
use std::time::Duration;
use std::{env::current_dir, time::Instant};

#[derive(PartialEq, Debug)]
pub enum MetronomeState {
    Start,
    Stop,
    SetBpm(u32),
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
    pub fn set_bpm(&self, bpm: u32) {
        let _ = self.sender.send(MetronomeState::SetBpm(bpm));
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
        let metronome_buffer = include_bytes!("../../assets/metronome.wav");
        let sample = load_wav_from_bytes(metronome_buffer).expect("Could not load metronome");
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
    pub fn set_bpm(&mut self, bpm: u32) {
        self.state = MetronomeState::Stop;
        self.bpm = bpm;
    }
}

impl Playable<f32> for Metronome {
    fn next_sample(&mut self) -> Option<(f32, f32)> {
        println!("{:?}", self.state);
        match self.state {
            MetronomeState::Start => self.sampler.next_sample(),
            MetronomeState::SetBpm(_) => None,
            MetronomeState::Stop => None,
        }
    }
}

pub fn run_metronome(mut metronome: Metronome) {
    let mut ticker = tick(Duration::from_secs_f64(60.0 / metronome.bpm as f64));

    std::thread::spawn(move || loop {
        if metronome.state == MetronomeState::Start {
            while let Some(sample) = metronome.sampler.next_sample() {
                let _ = metronome.sender.send(sample);
            }
            if let Ok(msg) = metronome.controller_receiver.try_recv() {
                match msg {
                    MetronomeState::Start => {
                        metronome.start();
                        ticker = tick(Duration::from_secs_f64(60.0 / metronome.bpm as f64));
                    }
                    MetronomeState::SetBpm(bpm) => {
                        metronome.set_bpm(bpm);
                        ticker = tick(Duration::from_secs_f64(60.0 / metronome.bpm as f64))
                    }
                    MetronomeState::Stop => metronome.stop(),
                }
            }
            if let Ok(_) = ticker.try_recv() {
                if metronome.state == MetronomeState::Start {
                    metronome.sampler.play_once();
                }
            }
            std::thread::sleep(Duration::from_millis(1));
        } else {
            select! {
                recv(metronome.controller_receiver) -> msg => {
                    match msg {
                        Ok(MetronomeState::Start) => metronome.start(),
                        Ok(MetronomeState::SetBpm(bpm)) => {
                            metronome.set_bpm(bpm);
                            ticker = tick(Duration::from_secs_f64(60.0 / metronome.bpm as f64))
                        },
                        Ok(MetronomeState::Stop) => metronome.stop(),
                        Err(err) => println!("{}", err)
                    }
                },
                recv(ticker) -> _ => {
                    if metronome.state == MetronomeState::Start {
                        metronome.sampler.play();
                    }
                }
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
