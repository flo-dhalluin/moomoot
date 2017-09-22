use super::Synth;
use params::{SynthParam, SynthParams, Parametrized};
use SoundSample;
use std::f64::consts::PI;


struct SineParams {
    pub amplitude: SynthParam,
    pub frequency: SynthParam,
}

impl SynthParams for SineParams {
    fn list_params(&self) -> Vec<&str> {
        vec!["amplitude", "frequency"]
    }

    fn set_param_value(&mut self, name: &str, value: SynthParam) {
        match name {
            "amplitude" => self.amplitude = value,
            "frequency" => self.frequency = value,
            _ => {} //
        }
    }
}

pub struct Sine {
    params: SineParams,
    time: f64,
    frame_t: f64,
}

impl Parametrized for Sine {
    fn get_params(&mut self) -> &mut SynthParams {
        return &mut self.params;
    }
}


impl Synth for Sine {
    fn new(frame_t: f64) -> Sine {
        Sine {
            time: 0.,
            frame_t: frame_t,
            params: SineParams {
                amplitude: SynthParam::DefaultValue(1.0),
                frequency: SynthParam::DefaultValue(440.0),
            },
        }
    }

    //    type Params = SineSynthParam;

    fn sample(&mut self) -> SoundSample {
        let x = self.params.frequency.value() * self.time * 2.0 * PI;
        self.time += self.frame_t;
        SoundSample::Sample(x.sin() * self.params.amplitude.value())
    }
}
