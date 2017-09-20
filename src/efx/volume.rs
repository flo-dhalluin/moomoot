use traits::SoundSample;
use params;
use super::Efx;


struct VolumeParams {
    volume: params::SynthParam
}

impl params::SynthParams for VolumeParams {
    fn list_params(&self) -> Vec<&str> {
        vec!["volume"]
    }

    fn set_param_value(&mut self, name: &str, value: params::SynthParam ) {
        match name {
            "volume" => self.volume = value,
            _ => {}
        }
    }
}

pub struct Volume {
    params: VolumeParams
}

impl params::Parametrized for Volume {
    fn get_params(&mut self) -> &mut params::SynthParams {
        &mut self.params
    }
}

impl Efx for Volume {

    fn new(vol: f64) -> Volume {
        Volume{params: VolumeParams{volume: params::SynthParam::DefaultValue(1.)}}
    }


    fn sample(&mut self, sample: f64) -> SoundSample {
        SoundSample::Sample( self.params.volume.value() * sample)
    }
}
