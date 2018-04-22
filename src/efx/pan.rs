use traits::{SoundSample, SampleValue, stereo_value};
use params::*;
use super::Efx;

// 0. => all right 1. => all left
declare_params!(PanParams { pan: 0.5 });

pub struct Pan {
    params: PanParams,
}

impl Parametrized for Pan {
    fn get_parameters(&mut self) -> &mut Parameters {
        &mut self.params
    }
}

impl Pan {
    pub fn new(params: PanParams) -> Pan {
        Pan { params: params }
    }
}

impl Efx for Pan {
    fn sample(&mut self, sample: SampleValue) -> SoundSample {

        let right_v = self.params.pan.value();
        let left_v = 1. - right_v;

        match sample {
            SampleValue::Mono(x) => stereo_value(x * right_v, x * left_v),
            SampleValue::Stereo(r, l) => stereo_value(right_v * r, left_v * l), // ??
        }
    }
}
