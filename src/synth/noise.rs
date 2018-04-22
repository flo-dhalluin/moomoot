//use super::super::Synth;

extern crate rand;
use self::rand::distributions::{Range, IndependentSample};
use synth::Synth;
use SoundSample;
use traits::mono_value;
use params::*;

/// white noise synth
pub struct WhiteNoise {
    rng: rand::XorShiftRng,
}

impl WhiteNoise {
    pub fn new() -> WhiteNoise {
        WhiteNoise { rng: rand::weak_rng() }
    }
}

impl Parametrized for WhiteNoise {}

impl Synth for WhiteNoise {
    fn sample(&mut self) -> SoundSample {
        let between = Range::new(-1f64, 1.);
        mono_value(between.ind_sample(&mut self.rng))
    }
}
