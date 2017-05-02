use base::Synth;
use std::mem;

struct LoopStep {
    // that's a freaking stack
    duration: f64,
    synth: Box<Synth>,
    next: Option<Box<LoopStep>>,
}

impl LoopStep {
    fn sample(&mut self, relative_time: f64, frame_t: f64) -> Option<f64> {
        if(relative_time < self.duration) {
            return Some(self.synth.sample(frame_t));
        }

        match self.next.as_mut() {
            Some(step) => return step.sample(relative_time - self.duration, frame_t),
            None => return None,
        }
    }
}

pub struct Looper {
    head: Option<Box<LoopStep>>,
    time: f64,
}


impl Looper {
    pub fn new() -> Looper {
        Looper {
            head: None,
            time: 0.0,
        }
    }

    pub fn add_step(&mut self, synth: Box<Synth>, duration: f64) {
        let mut dang = LoopStep {
            duration: duration,
            synth: synth,
            next: None,
        };
        mem::swap(&mut dang.next, &mut self.head);
        self.head = Some(Box::new(dang));
    }
}

impl Synth for Looper {

    fn sample(&mut self, frame_t: f64) -> f64 {
        self.time += frame_t;
        let time = self.time;

        let r = self.head.as_mut().and_then(|s| s.sample(time, frame_t));

        match r {
            Some(sample) => return sample,
            None => {
                self.time = 0.;
                return self.head.as_mut().and_then(|s| s.sample(0., frame_t)).unwrap();
            }
        }
    }

}
