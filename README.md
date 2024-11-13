# Looper Demo

This is a Tauri application, which uses a SolidJS frontend, combined with a Rust backend.
### Development Setup
  
First, install the tauri CLI, you can then run with tauri dev, the release flag is needed for performance, FunDSP sometimes overflows the stack as well if you don't pass the release flag.

```
cargo install tauri-cli
cargo tauri dev --release
```
### Known Issues / Planned Fixes

- Currently just waiting for the buffer to fill to loop, rather than using the clock. This causes the "clock" app to drift. When I start adding new instruments, midi, etc., I plan on having the app being more careful with time.

- Using FunDSP as a placeholder for a custom DSP setup, lots of weird traits if I wanted to turn the original iterator based track and mixer into an AudioNode (I originally followed an approach similar to Rodio's dynamic mixer, with a double buffer). Additionally, I had quite a bit of confusion as to how I could create these mixers at runtime, as the traits needed constant values. The audio graph has a lot of replicated code for the time being, and if anyone has a solution to get around the constants, or an example using an alternative runtime struct, that would be greatly appreciated. Additionally, I followed the example in the docs for their inputs, and decided to just send samples to the audio graph via crossbeam. This doesn't scale well and gives me much worse performance than the iterator/double buffer approach, but hey, placeholder effects!

- For the time being, I am just not going to allow the time to change during the performance. I don't really ever use this feature when I make music, and it is not worth the haste for me.

### Planned Features

- [ ] Basic audio graph and reverb implementation, would be nice to even have latency compensation, but I plan on just keeping the potential graphs small for the time being. This will then let me roll back to the double buffer recording approach, or maybe I continue the same multi input receiver setup and just use the iterators to build the mixers as opposed to sending samples via crossbeam or a ringbuffer. I was able to get pretty much as many tracks as possible with this approach before I had any latency or popping issues.
- [ ] Create a more structured event loop based on a central clock.
- [ ] Basic synthesizer, perhaps a 5 voice synth with FM capabilities, ADSR, tunable voices, sin, square, saw, etc. LFOs that can map to different parameters would also be nice
- [ ] Midi in, midi looping, clock out.
- [ ] Once I have these features in place, I plan on profiling quite a bit to hopefully get this to work on something like a RaspberryPI 4/5.
- [ ] Audio export, this really shouldn't be too hard. Naively I just want to spin up a thread to pass samples too and the app will just write, should not be too hard.
- [ ] Support for loading samples of different audio formats than f32 .wav.
- [ ] Sampler view with ADSR, setting different points, pitching up and down. I probably want to tackle this after the audio graph so I can reuse anything found here.
- [ ] A master compressor would be nice, I want an emulation or algorithm that has a bit of character.