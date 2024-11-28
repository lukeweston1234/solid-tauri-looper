use super::{audio_sample::AudioSample, playable::Playable};

pub struct Sampler<T> {
    sample: Option<AudioSample<T>>,
    sample_length: usize,
    position: usize,
    is_playing: bool,
    is_looping: bool,
}

impl<T> Sampler<T>
where
    T: Copy,
{
    pub fn new(sample: Option<AudioSample<T>>) -> Self {
        let sample_length = sample.as_ref().map_or(0, |s| s.get_sample_size());
        Self {
            sample_length,
            sample,
            position: 0,
            is_playing: false,
            is_looping: true,
        }
    }
    // Note, we don't need pause, because we just don't increment the next sampler iterator
    pub fn play(&mut self) {
        self.is_playing = true;
    }
    pub fn play_once(&mut self) {
        self.position = 0;
        self.is_playing = true;
    }
    pub fn stop(&mut self) {
        self.is_playing = false;
        self.position = 0;
    }
    pub fn set_sample(&mut self, sample: AudioSample<T>) {
        self.sample_length = sample.get_sample_size();
        self.sample = Some(sample);
        self.position = 0;
    }
    pub fn clear_sample(&mut self) {
        self.sample = None;
        self.sample_length = 0;
        self.position = 0;
        self.is_playing = false;
    }
    pub fn reset_position(&mut self) {
        self.position = 0;
    }
    pub fn set_is_looping(&mut self, new_value: bool) {
        self.is_looping = new_value;
    }
}

impl<T> Playable<T> for Sampler<T>
where
    T: Copy,
{
    fn next_sample(&mut self) -> Option<(T, T)> {
        if !self.is_playing {
            return None;
        }
        if self.position >= self.sample_length {
            if self.is_looping {
                self.position = 0;
            } else {
                return None;
            }
        };

        if let Some(sample) = self.sample.as_mut() {
            if let Some(chunk) = sample.get_sample(self.position) {
                self.position += 2;
                return Some(chunk);
            }
        }

        None
    }
}
