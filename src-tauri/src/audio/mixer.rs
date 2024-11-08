use crossbeam_channel::Receiver;
use fundsp::hacker32::*;

#[derive(Clone)]
pub struct MixerNode<const ID: u64> {
    receiver: Receiver<(f32, f32)>,
    pub reverb_mix: Shared,
    pub gain: Shared,
}

impl<const ID: u64> MixerNode<ID> {
    pub fn new(receiver: Receiver<(f32, f32)>) -> Self {
        Self {
            receiver,
            reverb_mix: shared(0.0),
            gain: shared(1.0),
        }
    }
    pub fn get_gain(&self) -> Shared {
        self.gain.clone()
    }
    pub fn set_gain(&self, gain: f32) {
        self.gain.set_value(gain);
    }
    pub fn get_reverb_mix(&self) -> Shared {
        self.reverb_mix.clone()
    }
    pub fn set_reverb_mix(&self, reverb_mix: f32) {
        self.reverb_mix.set_value(reverb_mix);
    }
}

impl<const ID: u64> AudioNode for MixerNode<ID> {
    const ID: u64 = ID;
    type Inputs = U0;
    type Outputs = U2;

    #[inline]
    fn tick(&mut self, _: &Frame<f32, Self::Inputs>) -> Frame<f32, Self::Outputs> {
        let (left, right) = self.receiver.try_recv().unwrap_or((0.0, 0.0));
        [left, right].into()
    }
}