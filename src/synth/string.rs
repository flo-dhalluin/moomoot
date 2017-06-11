
use std::mem;
use std::f64::consts::PI;
use synth::noise::WhiteNoise;
use Synth;
use SoundSample;
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
pub struct KarplusStrong {
    time: f64,
    frame_t: f64,
    period: f64, // that's const.
    last_feedback: f64,
    alpha: f64,  // alpha for LP
    feedback_gain: f64,
    energy: f64,
    delay_line: FixedRingBuffer,
    noise_synt: WhiteNoise

}

impl KarplusStrong {
    // freq : the fundamental note
    // frame_t : because we need the goddam sampling rate ..
    // cutoff_freq :
    // sustain : gain of the feedback 0 : non sustain - 1: inifinte
    pub fn new(freq : f64, frame_t : f64, cutoff_freq : f64, sustain : f64) -> KarplusStrong {
        let period = 1. / freq;
        let line_length =  (period / frame_t) as usize;
        let click = 2. * PI * frame_t * cutoff_freq;
        println!("clck : {} | {}", click, cutoff_freq);
        KarplusStrong {
            time: 0.,
            frame_t: frame_t,
            period: period,
            last_feedback: 0.,
            alpha: click / ( click + 1.),
            feedback_gain: sustain,
            energy: 0.,
            delay_line : FixedRingBuffer::from(vec![0.; line_length]),
            noise_synt : WhiteNoise::new()
        }
    }
}

impl Synth for KarplusStrong {

    fn sample(&mut self) -> SoundSample {
        let mut current_sample = self.last_feedback;

        if(self.time < self.period) {
            match self.noise_synt.sample() {
                SoundSample::Sample(n) => {current_sample += n;}
                SoundSample::Done => {}
            }
        }

        let res = current_sample;

        self.time += self.frame_t;
        // delay
        self.delay_line.queue(&mut current_sample);
        //println!("delay: {}", current_sample);
        // LP (first order )
        self.last_feedback = self.alpha * current_sample + (1. - self.alpha) * self.last_feedback;
        self.last_feedback *= self.feedback_gain;
        //self.last_feedback = current_sample * 0.9;
        let sq = self.last_feedback * self.last_feedback;
        self.energy = 0.95 * self.energy + 0.05 * sq;

        if(self.energy < 1e-9) {
            return SoundSample::Done;
        }
        SoundSample::Sample(res)
    }
}
