use crate::audio::audio_graph::build_audio_graph;
use crate::audio::stream::{build_input_device, build_output_device};
use crate::audio::{mixer::MixerNode, track::build_track, track::run_track};
use crossbeam_channel::bounded;
use fundsp::hacker32::*;
use std::env;
use tauri::AppHandle;

use super::app_controller::{build_app, App, AppController, MixerNodeEnum};

pub fn build_runtime() -> (AppController, App) {
    env::set_var("RUST_BACKTRACE", "full");

    // Sender / receiver for left and right channels (stereo mic).
    let (audio_input_sender, audio_input_receiver) = bounded(4096);

    let (next_looper_sender, next_looper_receiver) = bounded(10);

    let (track_one_controller, track_one, track_one_receiver) =
        build_track(audio_input_receiver.clone(), next_looper_sender.clone());
    let (track_two_controller, track_two, track_two_receiver) =
        build_track(audio_input_receiver.clone(), next_looper_sender.clone());
    let (track_three_controller, track_three, track_three_receiver) =
        build_track(audio_input_receiver.clone(), next_looper_sender.clone());
    let (track_four_controller, track_four, track_four_receiver) =
        build_track(audio_input_receiver.clone(), next_looper_sender.clone());
    let (track_five_controller, track_five, track_five_receiver) =
        build_track(audio_input_receiver.clone(), next_looper_sender.clone());
    let (track_six_controller, track_six, track_six_receiver) =
        build_track(audio_input_receiver.clone(), next_looper_sender.clone());

    let mixer_one = An(MixerNode::<1>::new(track_one_receiver));
    let mixer_two = An(MixerNode::<2>::new(track_two_receiver));
    let mixer_three = An(MixerNode::<3>::new(track_three_receiver));
    let mixer_four = An(MixerNode::<4>::new(track_four_receiver));
    let mixer_five = An(MixerNode::<5>::new(track_five_receiver));
    let mixer_six = An(MixerNode::<6>::new(track_six_receiver));
    // let master_bus = BusNode::new(mixer_one, mixer_two, mixer_three, mixer_four);

    run_track(track_one);
    run_track(track_two);
    run_track(track_three);
    run_track(track_four);
    run_track(track_five);
    run_track(track_six);

    let master_bus = build_audio_graph(
        mixer_one.clone(),
        mixer_two.clone(),
        mixer_three.clone(),
        mixer_four.clone(),
        mixer_five.clone(),
        mixer_six.clone(),
    );

    build_input_device(audio_input_sender);

    build_output_device(BlockRateAdapter::new(master_bus));

    let mixers: Vec<MixerNodeEnum> = vec![
        MixerNodeEnum::MixerOne(mixer_one),
        MixerNodeEnum::MixerTwo(mixer_two),
        MixerNodeEnum::MixerThree(mixer_three),
        MixerNodeEnum::MixerFour(mixer_four),
        MixerNodeEnum::MixerFive(mixer_five),
        MixerNodeEnum::MixerSix(mixer_six),
    ];

    let track_controllers = vec![
        track_one_controller,
        track_two_controller,
        track_three_controller,
        track_four_controller,
        track_five_controller,
        track_six_controller,
    ];

    let (app_controller, app) = build_app(mixers, track_controllers, next_looper_receiver);

    (app_controller, app)
}
