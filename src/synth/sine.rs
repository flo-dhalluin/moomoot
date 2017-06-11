use Synth;
use SoundSample;
use std::f64::consts::PI;

pub struct Sine {
    time: f64,
    freq: f64,
    frame_t: f64,
}

impl Sine {
    pub fn new(sample_rate: f64, freq: f64) -> Sine {
        Sine{
            time: 0.,
            frame_t: 1./sample_rate,
            freq: freq
        }
    }
}

impl Synth for Sine {

    fn sample(&mut self) -> SoundSample {
        let x = self.freq * self.time * 2.0 * PI;
        self.time += self.frame_t;
        SoundSample::Sample(x.sin())
    }
}
