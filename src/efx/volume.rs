use traits::{SoundSample, SampleValue, mono_value, stereo_value};
use params::*;
use super::Efx;


declare_params!(VolumeParams { volume: 1.0 });

pub struct Volume {
    params: VolumeParams,
}

impl Parametrized for Volume {
    fn get_parameters(&mut self) -> &mut Parameters {
        &mut self.params
    }
}


impl Volume {
    pub fn new(params: VolumeParams) -> Volume {
        Volume { params: params }
    }
}

impl Efx for Volume {
    fn sample(&mut self, sample: SampleValue) -> SoundSample {
        match sample {
            SampleValue::Mono(x) => mono_value(self.params.volume.value() * x),
            SampleValue::Stereo(r, l) => {
                stereo_value(
                    self.params.volume.value() * r,
                    self.params.volume.value() * l,
                )
            }
        }
    }
}
