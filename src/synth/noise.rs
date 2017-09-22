//use super::super::Synth;

extern crate rand;
use self::rand::distributions::{Range, IndependentSample};
use Synth;
use SoundSample;
use super::Parametrized;

pub struct WhiteNoise {
    rng: rand::XorShiftRng,
}

impl Parametrized for WhiteNoise {}

impl Synth for WhiteNoise {
    fn new(frame_t: f64) -> WhiteNoise {
        WhiteNoise { rng: rand::weak_rng() }
    }

    fn sample(&mut self) -> SoundSample {
        let between = Range::new(-1f64, 1.);
        SoundSample::Sample(between.ind_sample(&mut self.rng))
    }
}
