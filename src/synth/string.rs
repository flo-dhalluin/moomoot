
use std::mem;
use std::f64::consts::PI;
use synth::noise::WhiteNoise;
use Synth;
use traits::mono_value;
use traits::SampleValue;
use SoundSample;
use params::*;

// Karplus-Strong alg.
// https://en.wikipedia.org/wiki/Karplus%E2%80%93Strong_string_synthesis

// WhiteNoise  ->  +  ->  ->
//                 |      |
//                 LP  <- delay


// for the delay line ..
struct FixedRingBuffer {
    queue: Box<[f64]>,
    idx: usize, // index of last input ( so output is right behind)
}


impl FixedRingBuffer {
    #[inline]
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    // actually queue and dequeue ..
    pub fn queue(&mut self, elem: &mut f64) {
        let len = self.len();
        self.idx = (self.idx + len - 1) % len;
        //println!(" queue idx : {} / {} <= {}", self.idx, len, elem);
        mem::swap(unsafe { self.queue.get_unchecked_mut(self.idx) }, elem);
    }

    pub fn set_all(&mut self, elem: f64) {
        for e in self.queue.iter_mut() {
            *e = elem;
        }
    }
}

impl From<Vec<f64>> for FixedRingBuffer {
    fn from(vec: Vec<f64>) -> Self {
        debug_assert!(vec.len() > 0);
        FixedRingBuffer {
            queue: vec.into_boxed_slice(),
            idx: 0,
        }
    }
}

declare_params!(KarplusStrongParams { cutoff_freq: 6000.0, base_freq: 440.0, feedback_gain: 0.999});

pub struct KarplusStrong {
    params: KarplusStrongParams,
    time: f64,
    frame_t: f64,
    last_feedback: f64,
    energy: f64,
    delay_line: FixedRingBuffer,
    noise_synt: WhiteNoise,
}

impl KarplusStrong {
    // freq : the fundamental note
    // frame_t : because we need the goddam sampling rate ..
    // cutoff_freq :
    // sustain : gain of the feedback 0 : non sustain - 1: inifinte
    pub fn new(params: KarplusStrongParams) -> KarplusStrong {
        KarplusStrong {
            params: params,
            time: 0.,
            frame_t: 0.,
            last_feedback: 0.,
            energy: 1.,
            delay_line: FixedRingBuffer::from(Vec::new()),
            noise_synt: WhiteNoise::new(),
        }

    }

    fn update_delay_line(&mut self) -> usize {
        let line_len_f = (1. / (self.params.base_freq.value() * self.frame_t));
        let line_len = line_len_f as usize + 1;
        if line_len != self.delay_line.len() {
            self.delay_line = FixedRingBuffer::from(vec![0.; line_len]);
        }
        line_len

    }
}

impl Parametrized for KarplusStrong {
    fn get_parameters(&mut self) -> &mut Parameters {
        &mut self.params
    }
}

impl Synth for KarplusStrong {
    fn init(&mut self, frame_t: f64) {
        self.frame_t = frame_t;
        self.update_delay_line();
    }

    fn sample(&mut self) -> SoundSample {
        let mut current_sample = self.last_feedback;
        {
            let period = self.update_delay_line();
            if self.time < (period as f64) * self.frame_t {
                if let SoundSample::Sample(smp) = self.noise_synt.sample() {
                    if let SampleValue::Mono(n) = smp {
                        current_sample += n;
                    }
                }
            } else {
                if (self.energy < 1e-9) {
                    return SoundSample::Done;
                }
            }
        }

        let res = current_sample;

        self.time += self.frame_t;
        // delay
        self.delay_line.queue(&mut current_sample);
        // current_sample now is equal to "head" of the ring buffer.

        let alpha = {
            let p = 2. * PI * self.frame_t * self.params.cutoff_freq.value();
            p / (p + 1.)
        };

        self.last_feedback = alpha * current_sample + (1. - alpha) * self.last_feedback;
        self.last_feedback *= self.params.feedback_gain.value();
        //self.last_feedback = current_sample * 0.9;
        let sq = self.last_feedback * self.last_feedback;
        self.energy = 0.95 * self.energy + 0.05 * sq;

        mono_value(res)
    }
}
