
use std::mem;
use std::f64::consts::PI;
use synth::noise::WhiteNoise;
use Synth;
use SoundSample;
use synth::{SynthParam, SynthParams, Parametrized};

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
	pub fn len(&self) -> usize{self.queue.len()}

    // actually queue and dequeue ..
	pub fn queue(&mut self, elem: &mut f64) {
		let len = self.len();
		self.idx = (self.idx + len - 1) % len;
        //println!(" queue idx : {} / {} <= {}", self.idx, len, elem);
		mem::swap(unsafe{self.queue.get_unchecked_mut(self.idx)}, elem);
	}

    pub fn set_all(&mut self, elem: f64) {
        for e in self.queue.iter_mut() {
            *e = elem;
        }
    }
}

impl From<Vec<f64>> for FixedRingBuffer{
	fn from(vec: Vec<f64>) -> Self{
		debug_assert!(vec.len() > 0);
		FixedRingBuffer{
			queue: vec.into_boxed_slice(),
			idx: 0
		}
	}
}

struct KarplusStrongParams {
    cutoff_freq: SynthParam,
    base_freq: SynthParam,
    feedback_gain: SynthParam // sustain 1 = inifinite ..
}

impl SynthParams for KarplusStrongParams {

    fn list_params(&self) -> Vec<&str> {
        vec!["cutoff_freq", "base_freq", "feedback_gain"]
    }

    fn set_param_value(&mut self, param_id: &str, value: SynthParam) {
        match param_id {
            "cutoff_freq" => self.cutoff_freq = value,
            "base_freq" => self.base_freq = value,
            "feedback_gain" => self.feedback_gain = value,
            _ => {},
        }
    }
}

pub struct KarplusStrong {
    params: KarplusStrongParams,
    time: f64,
    frame_t: f64,
    last_feedback: f64,
    energy: f64,
    delay_line: FixedRingBuffer,
    noise_synt: WhiteNoise

}

impl KarplusStrong {
    // freq : the fundamental note
    // frame_t : because we need the goddam sampling rate ..
    // cutoff_freq :
    // sustain : gain of the feedback 0 : non sustain - 1: inifinte

    fn update_delay_line(&mut self) -> usize {
        let line_len_f = (1. / ( self.params.base_freq.value() * self.frame_t));
        let line_len = line_len_f as usize + 1;
        if line_len != self.delay_line.len() {
            self.delay_line = FixedRingBuffer::from(vec![0.; line_len]);
        }
        line_len

    }
}

impl Parametrized for KarplusStrong {
    fn get_params(&mut self) -> &mut SynthParams {
        &mut self.params
    }
}

impl Synth for KarplusStrong {

    fn new(frame_t : f64) -> KarplusStrong {

        let params = KarplusStrongParams {
            cutoff_freq: SynthParam::DefaultValue(6000.0),
            base_freq: SynthParam::DefaultValue(440.0),
            feedback_gain: SynthParam::DefaultValue(0.999),
        };

        KarplusStrong {
            params: params,
            time: 0.,
            frame_t: frame_t,
            last_feedback: 0.,
            energy: 1.,
            delay_line : FixedRingBuffer::from(Vec::new()),
            noise_synt : WhiteNoise::new(frame_t)
        }
    }

    fn sample(&mut self) -> SoundSample {
        let mut current_sample = self.last_feedback;
        {
            let period = self.update_delay_line();
            if self.time < (period as f64) * self.frame_t {
                if let SoundSample::Sample(n) = self.noise_synt.sample() {
                    current_sample += n;
                }
            }  else {
                if(self.energy < 1e-9) {
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
            let p = 2. * PI *self.frame_t*self.params.cutoff_freq.value();
            p / ( p + 1.)
        };

        self.last_feedback = alpha * current_sample + (1. - alpha) * self.last_feedback;
        self.last_feedback *= self.params.feedback_gain.value();
        //self.last_feedback = current_sample * 0.9;
        let sq = self.last_feedback * self.last_feedback;
        self.energy = 0.95 * self.energy + 0.05 * sq;

        SoundSample::Sample(res)
    }
}
