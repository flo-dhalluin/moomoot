use traits::SoundSample;
use params::*;
use super::Efx;


declare_params!(VolumeParams {volume: 1.0});

pub struct Volume {
    params: VolumeParams,
}

impl Parametrized for Volume {
    fn get_parameters(&mut self) -> &mut Parameters {
        &mut self.params
    }
}


impl Volume {
    pub fn new(params : VolumeParams) -> Volume {
        Volume{params: params}
    }
}

impl Efx for Volume {


    fn sample(&mut self, sample: f64) -> SoundSample {
        SoundSample::Sample(self.params.volume.value() * sample)
    }
}
