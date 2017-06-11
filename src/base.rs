extern crate jack;
use self::jack::prelude::{AudioOutPort, AudioOutSpec, Client, JackControl, ProcessHandler,
                    ProcessScope, Port};


/** output for a synth either a sample, or silence (done trigger autoremove from the tree )*/
pub enum SoundSample {
    Sample(f64),
    Done,
}


pub trait Synth {
    fn sample(&mut self) -> SoundSample;
}

pub trait Efx {
    fn sample(&mut self, sample: f64) -> SoundSample;
}

// combinators

struct Mixer {
    synths: Vec<Box<Synth>>
}

impl Mixer {

    pub fn new() -> Mixer {
        Mixer { synths: Vec::new() }
    }

    pub fn add_synth(&mut self, s: Box<Synth>) {
        self.synths.push(s);
    }
}

impl Synth for Mixer {
    fn sample(&mut self) -> SoundSample {
        let mut result:f64 = 0.;
        let mut actives = 0;
        let mut v: Vec<usize> = Vec::new();
        for (idx, s) in self.synths.iter_mut().enumerate() {
            match s.sample() {
                SoundSample::Sample(x) => { result += x; actives += 1 },
                SoundSample::Done => {v.push(idx);}
            }
        }
        for idx in v {
            self.synths.remove(idx);
        }

        if(actives > 0 ) {
            return SoundSample::Sample(result);
        }
        SoundSample::Done
    }
}


pub struct MooMoot {
    out_port : Port<AudioOutSpec>,
    root_mixer : Mixer,
    sample_rate : f64,
}

impl MooMoot {
    pub fn new(client: &Client) -> MooMoot {
        let port = client.register_port("moomoot", AudioOutSpec::default()).unwrap();
        let sample_rate = client.sample_rate();
        MooMoot {
            out_port: port,
            sample_rate: sample_rate as f64,
            root_mixer: Mixer::new()
        }
    }

    // add to root mixer
    pub fn add_synth(&mut self, s: Box<Synth>) {
        self.root_mixer.add_synth(s);
    }
}

impl ProcessHandler for MooMoot {
    fn process(&mut self, _:&Client, ps: &ProcessScope) -> JackControl {
        // Get output buffer
        let mut out_p = AudioOutPort::new(&mut self.out_port, ps);
        let out: &mut [f32] = &mut out_p;

        // Check frequency requests
        /*while let Ok(f) = rx.try_recv() {
            time = 0.0;
            frequency = f;
        }*/

        // Write output
        for v in out.iter_mut() {
            match self.root_mixer.sample() {
                SoundSample::Sample(s) => {*v = s as f32;}
                Done => {*v = 0.;}
            }
        }

        // Continue as normal
        JackControl::Continue
    }
}
