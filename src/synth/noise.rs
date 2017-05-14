use super::super::Synth;

extern crate rand;
use self::rand::distributions::{Range, IndependentSample};

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
    fn reset(&mut self) {}
    
    fn sample(&mut self, frame_t: f64) -> f64 {
        let between = Range::new(-1f64, 1.);
        between.ind_sample(&mut self.rng)
    }
}
