use super::{
    metronome::{self, Metronome},
    mixer::MixerNode,
};
use fundsp::hacker32::*;

// TODO, Figure out how to make these at runtime in FunDSP

pub fn build_audio_graph(
    mixer_one: An<MixerNode<1>>,
    mixer_two: An<MixerNode<2>>,
    mixer_three: An<MixerNode<3>>,
    mixer_four: An<MixerNode<4>>,
    mixer_five: An<MixerNode<5>>,
    mixer_six: An<MixerNode<6>>,
    mixer_feedback: An<MixerNode<7>>,
) -> (Box<dyn AudioUnit>, Shared, Shared) {
    let reverb = reverb2_stereo(20.0, 3.0, 1.0, 0.2, highshelf_hz(1000.0, 1.0, db_amp(-1.0)));
    let chorus = chorus(0, 0.0, 0.03, 0.2) | chorus(1, 0.0, 0.03, 0.2);

    let mx_one_wet = mixer_one.get_reverb_mix();
    let mx_one_gain = mixer_one.get_gain();

    let mixer_one_processed = mixer_one
        >> ((var(&mx_one_wet) | var(&mx_one_wet)) * (reverb.clone() >> chorus.clone())
            & ((1.0 - var(&mx_one_wet)) | (1.0 - var(&mx_one_wet))) * multipass())
        >> multipass() * (var(&mx_one_gain) | var(&mx_one_gain));

    let mx_two_wet = mixer_two.get_reverb_mix();
    let mx_two_gain = mixer_two.get_gain();

    let mixer_two_processed = mixer_two
        >> ((var(&mx_two_wet) | var(&mx_two_wet)) * (reverb.clone() >> chorus.clone())
            & ((1.0 - var(&mx_two_wet)) | (1.0 - var(&mx_two_wet))) * multipass())
        >> multipass() * (var(&mx_two_gain) | var(&mx_two_gain));

    let mx_three_wet = mixer_three.get_reverb_mix();
    let mx_three_gain = mixer_three.get_gain();

    let mixer_three_processed = mixer_three
        >> ((var(&mx_three_wet) | var(&mx_three_wet)) * (reverb.clone() >> chorus.clone())
            & ((1.0 - var(&mx_three_wet)) | (1.0 - var(&mx_three_wet))) * multipass())
        >> multipass() * (var(&mx_three_gain) | var(&mx_three_gain));

    let mx_four_wet = mixer_four.get_reverb_mix();
    let mx_four_gain = mixer_four.get_gain();

    let mixer_four_processed = mixer_four
        >> ((var(&mx_four_wet) | var(&mx_four_wet)) * (reverb.clone() >> chorus.clone())
            & ((1.0 - var(&mx_four_wet)) | (1.0 - var(&mx_four_wet))) * multipass())
        >> multipass() * (var(&mx_four_gain) | var(&mx_four_gain));

    let mx_five_wet = mixer_five.get_reverb_mix();
    let mx_five_gain = mixer_five.get_gain();

    let mixer_five_processed = mixer_five
        >> ((var(&mx_five_wet) | var(&mx_five_wet)) * (reverb.clone() >> chorus.clone())
            & ((1.0 - var(&mx_five_wet)) | (1.0 - var(&mx_five_wet))) * multipass())
        >> multipass() * (var(&mx_five_gain) | var(&mx_five_gain));

    let mx_six_wet = mixer_six.get_reverb_mix();
    let mx_six_gain = mixer_six.get_gain();

    let mixer_six_processed = mixer_six
        >> ((var(&mx_six_wet) | var(&mx_six_wet)) * (reverb.clone() >> chorus.clone())
            & ((1.0 - var(&mx_six_wet)) | (1.0 - var(&mx_six_wet))) * multipass())
        >> multipass() * (var(&mx_six_gain) | var(&mx_six_gain));

    let mx_feedback_wet = mixer_feedback.get_reverb_mix();
    let mx_feedback_gain = mixer_feedback.get_gain();

    let mixer_feedback_processed = mixer_feedback
        >> ((var(&mx_feedback_wet) | var(&mx_feedback_wet)) * (reverb.clone() >> chorus.clone())
            & ((1.0 - var(&mx_feedback_wet)) | (1.0 - var(&mx_feedback_wet))) * multipass())
        >> multipass() * (var(&mx_feedback_gain) | var(&mx_feedback_gain));

    let master_reverb = shared(0.6);
    let master_gain = shared(0.7);

    let master_bus = (mixer_one_processed
        + mixer_two_processed
        + mixer_three_processed
        + mixer_four_processed
        + mixer_five_processed
        + mixer_six_processed
        + mixer_feedback_processed)
        >> ((var(&master_reverb) | var(&master_reverb)) * (reverb.clone() >> chorus.clone())
            & ((1.0 - var(&master_reverb)) | (1.0 - var(&master_reverb))) * multipass())
        >> multipass() * (var(&master_gain) | var(&master_gain));

    (Box::new(master_bus), master_gain, master_reverb)
}
