extern crate moomoot;

use moomoot::MooMoot;
use moomoot::synth::string::{KarplusStrong, KarplusStrongParams};
use moomoot::synth::noise::WhiteNoise;
use moomoot::synth::sine::{Sine, SineParams};
use moomoot::efx::volume::{Volume, VolumeParams};
use moomoot::efx::pan::{Pan, PanParams};
use moomoot::param_expression;
use std::{time, thread};


fn main() {

    let mut m = MooMoot::start();

    let root = m.root_mixer();
    let blah = m.add_mixer(&root, "blah");
    let noise = m.add_mixer(&root, "noiz");

    // white noise beeing parameter less ..
    m.add_synth(&noise, WhiteNoise::new());
    m.add_efx(&noise, Volume::new(VolumeParams::default().volume(0.2)));
    m.add_efx(&noise, Pan::new(PanParams::default().pan("noize_pan")));

    m.add_efx(&blah, Pan::new(PanParams::default()));

    let mut note: f64 = 1.0;
    let mut random: u64 = 852;

    let sines = m.add_mixer(&blah, "sines");
    let strings = m.add_mixer(&blah, "strings");

    for i in 1..10 {
        let freq_s = format!("freq * {}", f64::from(i * i)*1.1);
        println!("{} .", freq_s);
        m.add_synth(&sines,
            Sine::new(SineParams::default().frequency(
                param_expression::parse_param_expression(&freq_s).unwrap()
            ).amplitude( 1. / f64::from(i)))
        );
    }

    m.add_efx(&sines, Volume::new(VolumeParams::default().volume(0.1)));

    loop {

        m.add_synth(
            &strings,
            KarplusStrong::new(KarplusStrongParams::default().base_freq(note * 75.0)),
        );

        thread::sleep(time::Duration::from_millis(250));

        m.set_bus_value("freq", note * 50.0);
        m.set_bus_value("noize_pan", 0.5 + note.cos() * 0.5);

        thread::sleep(time::Duration::from_millis(250 + random));

        random = (random * 17) % 634 + 13;

        note *= 1.3;

        if note > 20.0 {
            note = 1.0;
        }
    }


}
