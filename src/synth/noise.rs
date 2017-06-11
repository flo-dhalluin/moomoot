//use super::super::Synth;

extern crate rand;
use self::rand::distributions::{Range, IndependentSample};
use Synth;
use SoundSample;

pub struct WhiteNoise {
    rng: rand::XorShiftRng
}

impl WhiteNoise {
    pub fn new() -> WhiteNoise {
        WhiteNoise{
            rng: rand::weak_rng()
        }
    }
}

impl Synth for WhiteNoise {

    fn sample(&mut self) -> SoundSample {
        let between = Range::new(-1f64, 1.);
        SoundSample::Sample(between.ind_sample(&mut self.rng))
    }
}
