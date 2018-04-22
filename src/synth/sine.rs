use super::Synth;
use params::*;
use traits::mono_value;
use SoundSample;

use std::f64::consts::PI;


declare_params!(SineParams {
    amplitude: 1.0,
    frequency: 440.0
});

/// Pure sine synth.
pub struct Sine {
    params: SineParams,
    time: f64,
    frame_t: f64,
}

impl Parametrized for Sine {
    fn get_parameters(&mut self) -> &mut Parameters {
        return &mut self.params;
    }
}

impl Sine {
    pub fn new(params: SineParams) -> Sine {
        Sine {
            time: 0.0,
            frame_t: 0.0,
            params: params,
        }
    }
}

impl Synth for Sine {
    fn init(&mut self, frame_t: f64) {
        self.frame_t = frame_t;
    }
    //    type Params = SineSynthParam;

    fn sample(&mut self) -> SoundSample {
        let x = self.params.frequency.value() * self.time * 2.0 * PI;
        self.time += self.frame_t;
        mono_value(x.sin() * self.params.amplitude.value())
    }
}
