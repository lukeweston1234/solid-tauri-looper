use super::{audio_sample::AudioSample, playable::Playable, sampler::Sampler};
use crossbeam_channel::{bounded, unbounded, Receiver, Sender};

#[derive(PartialEq)]
enum TrackState {
    Playing,
    Paused,
    OnlyInput,
    Stopped,
    Recording,
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
        self.sender.send(TrackState::Playing);
    }
    pub fn pause(&self) {
        self.sender.send(TrackState::Paused);
    }
    pub fn stop(&self) {
        self.sender.send(TrackState::Stopped);
    }
    pub fn record(&self) {
        self.sender.send(TrackState::Recording);
    }
    pub fn only_input(&self) {
        self.sender.send(TrackState::OnlyInput);
    }
    pub fn end(&self) {
        self.sender.send(TrackState::End);
    }
}

pub struct Track<T>
where
    T: Send + Copy,
{
    audio_sender: Sender<(T, T)>,
    input_receiver: Receiver<(T, T)>,
    controller_receiver: Receiver<TrackState>,
    next_loop_sender: Sender<()>,
    state: TrackState,
    sampler: Sampler<T>,
    recording_clip: Option<Vec<T>>,
    initial_vec_size: usize, // We will potentially use this again when recording
}
impl<T> Track<T>
where
    T: Send + Copy,
{
    pub fn new(
        audio_sender: Sender<(T, T)>,
        input_receiver: Receiver<(T, T)>,
        controller_receiver: Receiver<TrackState>,
        next_loop_sender: Sender<()>,
        sampler: Sampler<T>,
        recording_clip: Option<Vec<T>>,
        initial_vec_size: usize,
    ) -> Self {
        Self {
            audio_sender,
            input_receiver,
            controller_receiver,
            next_loop_sender,
            state: TrackState::Stopped,
            sampler: sampler,
            recording_clip: Some(Vec::with_capacity(initial_vec_size)),
            initial_vec_size,
        }
    }
    pub fn handle_controller_messages(&mut self) {
        if let Ok(new_state) = self.controller_receiver.try_recv() {
            if new_state == TrackState::Stopped {
                self.sampler.reset_position();
            }
            self.state = new_state;
        }
    }
    fn handle_recording(&mut self) {
        if let Ok(sample) = self.input_receiver.try_recv() {
            self.audio_sender.send(sample).unwrap();

            if self.state == TrackState::OnlyInput {
                return; // Don't worry about recording if we are in an input only loop
            }

            let clip = self
                .recording_clip
                .get_or_insert_with(|| Vec::with_capacity(self.initial_vec_size));

            clip.push(sample.0);
            clip.push(sample.1);

            if clip.len() >= self.initial_vec_size {
                self.add_clip();
            }
        }
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
}

pub fn run_track<T>(mut track: Track<T>)
where
    T: Send + Copy + 'static,
{
    std::thread::spawn(move || loop {
        track.handle_controller_messages();

        match track.state {
            TrackState::Recording | TrackState::OnlyInput => track.handle_recording(),
            TrackState::Playing => track.handle_playback(),
            TrackState::Paused | TrackState::Stopped => {}
            TrackState::End => break,
        }
    });
}

pub fn build_track(
    input_receiver: Receiver<(f32, f32)>,
    next_loop_sender: Sender<()>,

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
        sampler,
        None,
        705600, // 44100k, 60 bpm, 4 beats,
    );

    (track_controller, track, track_audio_receiver)
}
