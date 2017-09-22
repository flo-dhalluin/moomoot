extern crate moomoot;

use moomoot::MooMoot;
use moomoot::synth::string::KarplusStrong;
use moomoot::synth::noise::WhiteNoise;
use moomoot::synth::sine::Sine;
use moomoot::ParamValue;
use moomoot::efx::volume::Volume;
use std::{time, thread};


fn main() {

    let mut m = MooMoot::start();

    let root = m.root_mixer();
    let blah = m.add_mixer(&root, "blah");
    let noise = m.add_mixer(&root, "noiz");

    // white noise beeing parameter less ..
    m.add_synth::<WhiteNoise>(&noise, Vec::new());
    m.add_efx::<Volume>(
        &noise,
        vec![("volume".to_string(), ParamValue::Constant(0.1))],
    );

    let mut note: f64 = 1.0;
    let mut random: u64 = 852;
    let frequency_p = "frequency".to_string();
    m.add_synth::<Sine>(
        &blah,
        vec![
            (frequency_p, ParamValue::BusValue("freq".to_string())),
            ("amplitude".to_string(), ParamValue::Constant(0.4)),
        ],
    );

    loop {

        m.add_synth::<KarplusStrong>(
            &blah,
            vec![("base_freq".to_string(), ParamValue::Constant(note * 75.0))],
        );

        m.set_bus_value("freq", note * 50.0);

        thread::sleep(time::Duration::from_millis(250 + random));

        random = (random * 17) % 634 + 13;

        note *= 1.3;

        if note > 20.0 {
            note = 1.0;
        }
    }


}
