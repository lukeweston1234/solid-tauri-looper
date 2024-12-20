use std::default;

use super::{audio_sample::AudioSample, playable::Playable, sampler::Sampler};
use crossbeam_channel::{bounded, unbounded, Receiver, Sender};

#[derive(PartialEq, Clone)]
pub enum TrackState {
    Playing,
    Paused,
    OnlyInput,
    Stopped,
    Recording,
    ClearSample,
    RecomputeBufferSize(usize),
    End,
}

pub struct TrackController {
    sender: Sender<TrackState>,
}
impl TrackController {
    pub fn new(sender: Sender<TrackState>) -> Self {
        Self { sender }
    }
    pub fn play(&self) {
        let _ = self.sender.send(TrackState::Playing);
    }
    pub fn pause(&self) {
        let _ = self.sender.send(TrackState::Paused);
    }
    pub fn stop(&self) {
        let _ = self.sender.send(TrackState::Stopped);
    }
    pub fn record(&self) {
        let _ = self.sender.send(TrackState::Recording);
    }
    pub fn clear_sample(&self) {
        let _ = self.sender.send(TrackState::ClearSample);
    }
    pub fn only_input(&self) {
        let _ = self.sender.send(TrackState::OnlyInput);
    }
    pub fn end(&self) {
        let _ = self.sender.send(TrackState::End);
    }
    pub fn recompute_buffer_size(&self, buffer_size: usize) {
        let _ = self
            .sender
            .send(TrackState::RecomputeBufferSize(buffer_size));
    }
}

pub struct Track<T>
where
    T: Send + Copy + Into<f32>,
{
    audio_sender: Sender<(T, T)>,
    input_receiver: Receiver<(T, T)>,
    controller_receiver: Receiver<TrackState>,
    next_loop_sender: Sender<()>,
    audio_display_sender: Sender<f32>, // Send the average size of a buffer
    state: TrackState,
    sampler: Sampler<T>,
    recording_clip: Option<Vec<T>>,
    initial_vec_size: usize, // We will potentially use this again when recording,
    display_vec_chunk_size: usize,
    display_average_buffer: Vec<T>,
}
impl<T> Track<T>
where
    T: Send + Copy + Into<f32>,
{
    pub fn new(
        audio_sender: Sender<(T, T)>,
        input_receiver: Receiver<(T, T)>,
        controller_receiver: Receiver<TrackState>,
        next_loop_sender: Sender<()>,
        audio_display_sender: Sender<f32>,
        sampler: Sampler<T>,
        initial_vec_size: usize,
        display_vec_size: usize,
    ) -> Self {
        Self {
            audio_sender,
            input_receiver,
            controller_receiver,
            next_loop_sender,
            audio_display_sender,
            state: TrackState::Stopped,
            sampler,
            recording_clip: Some(Vec::with_capacity(initial_vec_size)),
            initial_vec_size,
            display_vec_chunk_size: display_vec_size,
            display_average_buffer: Vec::with_capacity(initial_vec_size / display_vec_size),
        }
    }

    pub fn recompute_buffer_size(&mut self, buffer_size: usize) {
        self.state = TrackState::Stopped;
        self.initial_vec_size = buffer_size;
        self.recording_clip = Some(Vec::with_capacity(buffer_size));
    }

    pub fn handle_controller_messages(&mut self, mut new_state: TrackState) {
        match (self.state.clone(), new_state.clone()) {
            (TrackState::Stopped, TrackState::Playing) => {
                self.sampler.play();
            }
            (_, TrackState::Stopped) => {
                self.recording_clip = None;
                self.sampler.stop();
            }
            (_, TrackState::ClearSample) => {
                self.clear_sample();
                self.display_average_buffer.clear();

                new_state = TrackState::Stopped;
            }
            (TrackState::Recording, TrackState::Playing) => {
                self.recording_clip = None;
                self.display_average_buffer.clear();
                new_state = TrackState::OnlyInput;
            }
            (TrackState::Paused, TrackState::Recording) => {
                // self.recording_clip = None;
                // self.display_average_buffer.clear();
            }
            (_, TrackState::RecomputeBufferSize(buffer_size)) => {
                self.recompute_buffer_size(buffer_size);
            }
            _ => (),
        }
        self.state = new_state;
    }

    fn handle_recording(&mut self) {
        if let Ok(sample) = self.input_receiver.try_recv() {
            self.audio_sender.send(sample).unwrap();

            if self.state == TrackState::OnlyInput {
                return; // Don't worry about recording if we are in an input only loop
            }

            {
                if let Some(clip) = self.recording_clip.as_mut() {
                    clip.push(sample.0);
                    clip.push(sample.1);
                    if clip.len() >= self.initial_vec_size {
                        self.add_clip();
                    }
                } else {
                    panic!("We can't be here!");
                }
            }

            self.handle_display_vec(sample);
        }
    }
    fn handle_display_vec(&mut self, sample: (T, T)) {
        if self.display_average_buffer.len() >= self.initial_vec_size / self.display_vec_chunk_size
        {
            let sum: f32 = self.display_average_buffer.iter().map(|&x| x.into()).sum();

            let average = sum / self.display_average_buffer.len() as f32;

            let _ = self.audio_display_sender.send(average);
            self.display_average_buffer.clear();
        }
        self.display_average_buffer.push(sample.0);
        self.display_average_buffer.push(sample.1);
    }
    fn add_clip(&mut self) {
        let _ = self.next_loop_sender.send(());
        let final_clip = self.recording_clip.take().unwrap();
        self.sampler
            .set_sample(AudioSample::new(final_clip, false, 44_100));
        self.state = TrackState::Playing;
        self.sampler.play();
    }
    fn handle_playback(&mut self) {
        if self.recording_clip.is_some() {
            self.add_clip();
        }
        if let Some(sample) = self.sampler.next_sample() {
            let _ = self.audio_sender.send(sample);
        }
    }
    fn clear_sample(&mut self) {
        self.sampler.clear_sample();
        self.recording_clip = Some(Vec::with_capacity(self.initial_vec_size));
        self.display_average_buffer.clear();
    }
}

pub fn run_track<T>(mut track: Track<T>)
where
    T: Send + Copy + Into<f32> + 'static,
{
    std::thread::spawn(move || loop {
        if track.state == TrackState::Stopped || track.state == TrackState::Paused {
            if let Ok(msg) = track.controller_receiver.recv() {
                track.handle_controller_messages(msg);
            }
        } else {
            if let Ok(msg) = track.controller_receiver.try_recv() {
                track.handle_controller_messages(msg);
            }
            match track.state {
                TrackState::Recording | TrackState::OnlyInput => track.handle_recording(),
                TrackState::Playing => track.handle_playback(),
                TrackState::Paused | TrackState::Stopped => (),
                TrackState::RecomputeBufferSize(buffer_size) => {
                    track.recompute_buffer_size(buffer_size)
                }
                TrackState::ClearSample => (),
                TrackState::End => break,
            }
        }
    });
}

pub fn build_track(
    input_receiver: Receiver<(f32, f32)>,
    next_loop_sender: Sender<()>,
    audio_display_sender: Sender<f32>,
    display_vec_chunk_size: usize,
    track_buffer_size: usize,
) -> (TrackController, Track<f32>, Receiver<(f32, f32)>) {
    let (track_state_sender, track_state_receiver) = unbounded::<TrackState>();

    let (track_audio_sender, track_audio_receiver) = bounded::<(f32, f32)>(4096);

    let track_controller = TrackController::new(track_state_sender);

    let sampler = Sampler::new(None);

    let track = Track::new(
        track_audio_sender,
        input_receiver,
        track_state_receiver,
        next_loop_sender,
        audio_display_sender,
        sampler,
        track_buffer_size, // 44100k, 60 bpm, 4 beats, 2 bars
        display_vec_chunk_size,
    );

    (track_controller, track, track_audio_receiver)
}
