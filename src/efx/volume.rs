use traits::*;

pub struct Volume {
    vol: f64
}

impl Efx for Volume {

    fn sample(&mut self, sample: f64) -> SoundSample {
        SoundSample::Sample( self.vol * sample)
    }
}

impl Volume {
    pub fn new(vol: f64) -> Volume {
        Volume{vol: vol}
    }
}
