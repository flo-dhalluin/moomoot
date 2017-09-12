extern crate moomoot;

use moomoot::MooMoot;
use moomoot::synth::string::KarplusStrong;
use moomoot::synth::noise::WhiteNoise;
use moomoot::efx::volume::Volume;
use std::{time, thread};


fn main() {

    let mut m = MooMoot::start();

    let srate = m.get_sampling_rate();

    m.add_mixer("root", "blah");

    m.add_mixer("root", "noiz");

    m.add_synth("noiz", WhiteNoise::new());
    m.add_efx("noiz", Volume::new(0.1));

    let mut note:f64 = 1.0;
    let mut random: u64 = 852;
    loop {

        m.add_synth("blah", KarplusStrong::new(note*50.0, 1./srate, 6000., 0.99));
        m.add_synth("blah", KarplusStrong::new(note*100.0, 1./srate, 6000., 0.99));


        thread::sleep(time::Duration::from_millis(250 + random));

        random = (random * 17) % 634 + 13;

        note *= 1.3;

        if note > 20.0 {
            note = 1.0;
        }
    }


}
