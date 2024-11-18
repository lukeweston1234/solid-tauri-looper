use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{default_host, FromSample, SizedSample};
use fundsp::hacker32::*;

use crossbeam_channel::{Receiver, Sender};

use crate::audio::metronome;
use crate::audio::playable::Playable;

use super::metronome::Metronome;

pub fn build_output_device(
    master_bus: BlockRateAdapter,
    metronome_buffer_receiver: Receiver<(f32, f32)>,
) {
    let host = cpal::default_host();

    // Start output.
    let out_device = host
        .default_output_device()
        .expect("Could not get default output device!");
    let out_config = out_device
        .default_output_config()
        .expect("Could not get default output config!");

    println!("Out channels: {:?}", out_config.channels());
    println!("Out sample rate: {:?}", out_config.sample_rate());
    println!("Out sample format: {:?}", out_config.sample_format());
    println!("Out sample format: {:?}", out_config.buffer_size());

    match out_config.sample_format() {
        cpal::SampleFormat::F32 => run_out::<f32>(
            &out_device,
            &out_config.into(),
            master_bus,
            metronome_buffer_receiver,
        ),
        cpal::SampleFormat::I16 => run_out::<i16>(
            &out_device,
            &out_config.into(),
            master_bus,
            metronome_buffer_receiver,
        ),
        cpal::SampleFormat::U16 => run_out::<u16>(
            &out_device,
            &out_config.into(),
            master_bus,
            metronome_buffer_receiver,
        ),
        format => eprintln!("Unsupported sample format: {}", format),
    }
}

pub fn build_input_device(sender: Sender<(f32, f32)>) {
    let host = default_host();
    // #[cfg(target_os = "windows")]
    // {
    //     host = cpal::host_from_id(cpal::HostId::Asio).expect("failed to initialise ASIO host");
    // }
    // #[cfg(target_os = "macos")]
    // {
    //     host = default_host();
    // }
    // #[cfg(target_os = "linux")]
    // {
    //     host = default_host();
    // }

    // Start input.
    let in_device = host
        .default_input_device()
        .expect("Could not get default input device!");
    let in_config = in_device
        .default_input_config()
        .expect("Could not get default input config!");

    println!("Int channels: {:?}", in_config.channels());
    println!("Int sample rate: {:?}", in_config.sample_rate());
    println!("Int sample format: {:?}", in_config.sample_format());
    println!("Int sample format: {:?}", in_config.buffer_size());

    match in_config.sample_format() {
        cpal::SampleFormat::F32 => run_in::<f32>(&in_device, &in_config.into(), sender),
        cpal::SampleFormat::I16 => run_in::<i16>(&in_device, &in_config.into(), sender),
        cpal::SampleFormat::I32 => run_in::<i32>(&in_device, &in_config.into(), sender),
        cpal::SampleFormat::U16 => run_in::<u16>(&in_device, &in_config.into(), sender),
        format => eprintln!("Unsupported sample format: {}", format),
    }
}

fn run_in<T>(device: &cpal::Device, config: &cpal::StreamConfig, sender: Sender<(f32, f32)>)
where
    T: SizedSample,
    f32: FromSample<T>,
{
    let channels = config.channels as usize;
    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
    let stream = device.build_input_stream(
        config,
        move |data: &[T], _: &cpal::InputCallbackInfo| read_data(data, channels, sender.clone()),
        err_fn,
        None,
    );
    if let Ok(stream) = stream {
        if let Ok(()) = stream.play() {
            std::mem::forget(stream);
        }
    }
    println!("Input stream built.");
}

fn read_data<T>(input: &[T], channels: usize, sender: Sender<(f32, f32)>)
where
    T: SizedSample,
    f32: FromSample<T>,
{
    for frame in input.chunks(channels) {
        let mut left = 0.0;
        let mut right = 0.0;
        for (channel, sample) in frame.iter().enumerate() {
            if channel & 1 == 0 {
                left = sample.to_sample::<f32>();
            } else {
                right = sample.to_sample::<f32>();
            }
        }
        if let Ok(()) = sender.try_send((left, right)) {}
    }
}

fn run_out<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    mut bus: BlockRateAdapter,
    metronome_buffer_receiver: Receiver<(f32, f32)>,
) where
    T: SizedSample + FromSample<f32>,
{
    let channels = config.channels as usize;

    bus.set_sample_rate(config.sample_rate.0 as f64);

    let mut next_value = move || {
        let (mut l, mut r) = bus.get_stereo();

        if let Ok(sample) = metronome_buffer_receiver.try_recv() {
            l += sample.0;
            r += sample.1;
        }

        return (l, r);
    };

    let err_fn = |err| eprintln!("An error occurred on stream: {}", err);
    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &mut next_value)
        },
        err_fn,
        None,
    );
    if let Ok(stream) = stream {
        if let Ok(()) = stream.play() {
            std::mem::forget(stream);
        }
    }
    println!("Output stream built.");
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> (f32, f32))
where
    T: SizedSample + FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        let sample = next_sample();
        let left = T::from_sample(sample.0);
        let right = T::from_sample(sample.1);

        for (channel, sample) in frame.iter_mut().enumerate() {
            if channel & 1 == 0 {
                *sample = left;
            } else {
                *sample = right;
            }
        }
    }
}
