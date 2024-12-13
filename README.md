# Looper Demo

[![Everything Is AWESOME](https://img.youtube.com/vi/tmZAew8MZ-Q&lc=UgwCs7TRIIKJUwdgtih4AaABAg/0.jpg)](https://www.youtube.com/watch?v=tmZAew8MZ-Q&lc=UgwCs7TRIIKJUwdgtih4AaABAg "Demo")

This is a Tauri application, which uses a SolidJS frontend, combined with a Rust backend.
### Development Setup

Once you have Rust and Cargo installed, you can follow the below.

First, install the tauri CLI, you can then run with tauri dev, the release flag is needed for performance, FunDSP sometimes overflows the stack as well if you don't pass the release flag.

#### Windows

For windows, you may have much lower latency from smaller buffers with ASIO (atleast from my experience).
[Install LLVM for the CPAL ASIO installer for windows here](https://github.com/llvm/llvm-project/releases/tag/llvmorg-18.1.8)

Note: You may want to double check and see if the branch you are on is using ASIO in the stream.rs file

You will want to use the LLVM-18.1.8-win64/32.exe installer, make sure to add it to your path

```
npm install
cargo install tauri-cli
cargo tauri dev --release
```

### Known Issues / Planned Fixes

- Using FunDSP as a placeholder for a custom DSP setup, lots of weird traits if I wanted to turn the original iterator based track and mixer into an AudioNode (I originally followed an approach similar to Rodio's dynamic mixer, with a double buffer). Additionally, I had quite a bit of confusion as to how I could create these mixers at runtime, as the traits needed constant values. The audio graph has a lot of replicated code for the time being, and if anyone has a solution to get around the constants, or an example using an alternative runtime struct, that would be greatly appreciated. Additionally, I followed the example in the docs for their inputs, and decided to just send samples to the audio graph via crossbeam. This gives much worse performance. There is another repo I have with a similiar backend using iterators and Freeverb instead, but that algorithm sounds noticably worse, and I will need to add some sort of attack to the reverb to prevent "bloom" or popping when the reverb is activated.

- For the time being, I am just not going to allow the time to change during the performance. I don't really ever use this feature when I make music, and it is not worth the hastle for me.

### Planned Features

- [ ] Refactor audio without using crossbeam for sending samples.
- [ ] Some sort of logic to load and sequence samples, as well as samples with different formats, i32, i24, etc.
- [ ] Create a more structured event loop based on a central clock.
- [ ] Basic synthesizer, perhaps a 5 voice synth with FM capabilities, ADSR, tunable voices, sin, square, saw, etc. LFOs that can map to different parameters would also be nice
- [ ] Midi in, midi looping, clock out.
- [ ] Once I have these features in place, I plan on profiling quite a bit to hopefully get this to work on something like a RaspberryPI 4/5.
- [ ] Audio export, this really shouldn't be too hard. Naively I just want to spin up a thread to pass samples too and the app will just write, should not be too hard.
- [ ] Support for loading samples of different audio formats than f32 .wav.
- [ ] Sampler view with ADSR, setting different points, pitching up and down. I probably want to tackle this after the audio graph so I can reuse anything found here.
- [ ] A master compressor would be nice, I want an algorithm that has a bit of character.
- [ ] SQLite support for saved sessions, saved themes, default themes, etc.
