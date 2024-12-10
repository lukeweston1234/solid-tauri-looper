use super::{playable::Playable, sampler::Sampler};
use crate::{app, audio::audio_sample::load_wav_from_bytes};
use crossbeam_channel::{bounded, select, tick, Receiver, Sender};
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

#[derive(PartialEq, Debug)]
pub enum MetronomeState {
    Start,
    Stop,
    SetBpm(u32),
    // SetMaximumBeat(u32), // Used to sync the front end ticker
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
        println!("In set bpm");
        let _ = self.sender.send(MetronomeState::SetBpm(bpm));
    }
    // pub fn set_maximum_beat(&self, maximum_beat: u32) {
    //     let _ = self
    //         .sender
    //         .send(MetronomeState::SetMaximumBeat(maximum_beat));
    // }
}

pub struct Metronome {
    state: MetronomeState,
    sender: Sender<(f32, f32)>,
    controller_receiver: Receiver<MetronomeState>,
    bpm: u32,
    sampler: Sampler<f32>,
    // maximum_beat: u32, // Used for sending a 'hey', we reset to the frontend for looping, to render the ticker
    // beat_count: u32,
}
impl Metronome {
    pub fn new(
        bpm: u32,
        sender: Sender<(f32, f32)>,
        controller_receiver: Receiver<MetronomeState>,
        // maximum_beat: u32,
    ) -> Self {
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
            // beat_count: 0,
            // maximum_beat,
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
        println!("In set_bpm actor");
        self.state = MetronomeState::Stop;
        self.bpm = bpm;
    }
    // pub fn set_maximum_beats(&mut self, maximum_beat: u32) {
    //     self.maximum_beat = maximum_beat * self.bpm
    // }
}

impl Playable<f32> for Metronome {
    fn next_sample(&mut self) -> Option<(f32, f32)> {
        println!("{:?}", self.state);
        match self.state {
            MetronomeState::Start => self.sampler.next_sample(),
            MetronomeState::SetBpm(_) => None,
            // MetronomeState::SetMaximumBeat(_) => None,
            MetronomeState::Stop => None,
        }
    }
}

pub fn run_metronome(mut metronome: Metronome, app_handle: AppHandle) {
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
                        println!("Setting new bpm: {}", bpm);
                        ticker = tick(Duration::from_secs_f64(60.0 / metronome.bpm as f64))
                    }
                    MetronomeState::Stop => metronome.stop(),
                    // MetronomeState::SetMaximumBeat(maximum_beat) => {
                    //     metronome.set_maximum_beats(maximum_beat)
                    // }
                }
            }
            if let Ok(_) = ticker.try_recv() {
                if metronome.state == MetronomeState::Start {
                    // if (metronome.beat_count == metronome.maximum_beat) {
                    //     metronome.beat_count = 0;
                    //     app_handle.emit("clock_reset", ());
                    // } else {
                    //     metronome.beat_count += 1;
                    // }
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
                            println!("Setting new bpm: {}", bpm);
                            metronome.set_bpm(bpm);
                            ticker = tick(Duration::from_secs_f64(60.0 / metronome.bpm as f64));
                        },
                        Ok(MetronomeState::Stop) => metronome.stop(),
                        // Ok(MetronomeState::SetMaximumBeat(maximum_beat)) => metronome.set_maximum_beats(maximum_beat),
                        Err(err) => println!("{}", err)
                    }
                },
                recv(ticker) -> _ => {
                    if metronome.state == MetronomeState::Start {
                        // if (metronome.beat_count == metronome.maximum_beat){
                        //     metronome.beat_count = 0;
                        // }
                        // else {
                        //     metronome.beat_count += 1;
                        // }
                        metronome.sampler.play();
                    }
                }
            }
        }
    });
}

pub fn build_metronome(
    bpm: u32,
    maximum_beat: u32,
) -> (Arc<MetronomeController>, Metronome, Receiver<(f32, f32)>) {
    let (controller_sender, controller_receiver) = bounded(10);

    let (metronome_buffer_sender, metronome_buffer_receiver) = bounded(512);

    let metronome_controller = MetronomeController::new(controller_sender);

    let metronome = Metronome::new(
        bpm,
        metronome_buffer_sender,
        controller_receiver,
        // maximum_beat,
    );

    (
        Arc::new(metronome_controller),
        metronome,
        metronome_buffer_receiver,
    )
}
