use Synth;
use std::f64::consts::PI;

pub struct Sine {
    time: f64,
    freq: f64,
}

impl Sine {
    pub fn new(freq: f64) -> Sine {
        Sine{
            time: 0.,
            freq: freq
        }
    }
}

impl Synth for Sine {

    fn reset(&mut self) {
        self.time = 0.;
    }

    fn sample(&mut self, frame_t: f64) -> f64 {
        let x = self.freq * self.time * 2.0 * PI;
        self.time += frame_t;
        x.sin()
    }
}
