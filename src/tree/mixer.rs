use synth::{Synth, Parametrized};
use efx::Efx;
use traits::SoundSample;
use std;

type MixerId = String;

// combinators
pub struct Mixer {

    is_transient: bool,
    synths: Vec<Box<Synth>>,
    pub sub_mixers: Vec<Box<Mixer>>,

    effects: Vec<Box<Efx>>,

    pub id: MixerId
}

impl Mixer {

    pub fn new( id: &str) -> Mixer {
        Mixer { is_transient: false,
                synths: Vec::new(),
                effects: Vec::new(),
                sub_mixers: Vec::new(),
                id:String::from(id) }
    }

    pub fn new_transient(id: &str) -> Mixer {
        let mut mixr = Mixer::new(id);
        mixr.is_transient = true;
        mixr
    }

    pub fn add_synth(&mut self, s: Box<Synth>) {
        self.synths.push(s);
    }

    pub fn add_efx(&mut self, efx: Box<Efx>) {
        self.effects.push(efx);
    }

    pub fn add_sub_mixer(&mut self, mixer: Mixer) {
        self.sub_mixers.push(Box::new(mixer));
    }

    pub fn mixer_count(&self) -> usize {

        let mut cnt = 1;

        for mix in self.sub_mixers.iter() {
            cnt += mix.mixer_count();
        }

        cnt
    }

    pub fn find_mixer(&mut self, id:&str) -> Option<&mut Mixer> {

        if id == self.id {
            Some(self)
        }
        else {
            self.sub_mixers.iter_mut()
                .find(|m| m.id == id)
                .map(|bxd| bxd.as_mut())
            }
    }
}


// them solution for downcast ..
trait AsSynth {
    fn as_synth(&mut self) -> &mut Synth;
}

impl AsSynth for Box<Synth> {
    fn as_synth(&mut self) -> &mut Synth  {
        self.as_mut()
    }
}

impl<T:Synth> AsSynth for Box<T> {
    fn as_synth(&mut self) -> &mut Synth {
        self.as_mut()
    }
}

// UGLY : see also drain_filter in nightly
fn sample_and_remove<S:AsSynth>(synths: &mut Vec<S>) -> SoundSample {
    let (samples, alive_synths):(Vec<_>, Vec<_>) = std::mem::replace(synths, Vec::new())
                .into_iter()
                .filter_map(|mut s| {
                    let sample = s.as_synth().sample();
                    match sample {
                        SoundSample::Done => None, // pop "Done" synths
                        _ => Some((sample, s))
                    }
                }).unzip();

    std::mem::replace(synths, alive_synths);
    samples.into_iter().sum()
}

impl Parametrized for Mixer {}

impl Synth for Mixer {

    fn new(frame_t: f64) -> Mixer {
        panic!("Mixer is not a synth")
    }

    fn sample(&mut self) -> SoundSample {

        let res = sample_and_remove(&mut self.synths)
                    + sample_and_remove(&mut self.sub_mixers);

        if let SoundSample::Sample(value) = res {
            let mut sample = value;
            for e in self.effects.iter_mut() {
                if let SoundSample::Sample(processed) = e.sample(sample) {
                    sample = processed;
                } else {
                    return SoundSample::Done;
                }
            }

            return SoundSample::Sample(sample);
        }
        if self.is_transient {
            SoundSample::Done
        } else {
            SoundSample::Silence
        }

    }
}
