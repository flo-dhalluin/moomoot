use synth::Synth;
use efx::Efx;
use traits::SoundSample;
use std::collections::LinkedList;
type MixerId = String;

// These are actually owning tree nodes.
// child lists are based on LinkedList because :
// - we don't need individual access ( traversal only )
// - we do need predictable node removal in the middle
// - we need actual O(1) append ( Real time remember ?? )
pub struct Mixer {
    is_transient: bool,
    synths: LinkedList<Box<Synth>>,
    pub sub_mixers: LinkedList<Box<Mixer>>,
    effects: Vec<Box<Efx>>,
    pub id: MixerId,
}

impl Mixer {
    pub fn new(id: &str) -> Mixer {
        Mixer {
            is_transient: false,
            synths: LinkedList::new(),
            effects: Vec::new(),
            sub_mixers: LinkedList::new(),
            id: String::from(id),
        }
    }

    pub fn new_transient(id: &str) -> Mixer {
        let mut mixr = Mixer::new(id);
        mixr.is_transient = true;
        mixr
    }

    pub fn add_synth(&mut self, s: Box<Synth>) {
        self.synths.push_back(s);
    }

    pub fn add_efx(&mut self, efx: Box<Efx>) {
        self.effects.push(efx);
    }

    pub fn add_sub_mixer(&mut self, mixer: Mixer) {
        self.sub_mixers.push_back(Box::new(mixer));
    }

    pub fn mixer_count(&self) -> usize {

        let mut cnt = 1;

        for mix in self.sub_mixers.iter() {
            cnt += mix.mixer_count();
        }

        cnt
    }

    pub fn find_mixer(&mut self, id: &str) -> Option<&mut Mixer> {

        if id == self.id {
            Some(self)
        } else {
            self.sub_mixers
                .iter_mut()
                .filter_map(|m| m.find_mixer(id))
                .next()
        }
    }
}


// them solution for downcast ..
trait AsSynth {
    fn sample(&mut self) -> SoundSample;
}

impl AsSynth for Box<Synth> {
    fn sample(&mut self) -> SoundSample {
        self.as_mut().sample()
    }
}

impl AsSynth for Box<Mixer> {
    fn sample(&mut self) -> SoundSample {
        self.as_mut().sample()
    }
}

// UGLY : see also drain_filter in nightly
fn sample_and_remove<S: AsSynth>(synths: &mut LinkedList<S>) -> SoundSample {

    let mut res = SoundSample::Silence;

    synths.drain_filter(|s| {
        let sample = s.sample();
        match sample {
            SoundSample::Done => true,
            _ => {
                res += sample;
                false
            }
        }
    });
    res
}

impl Mixer {
    pub fn sample(&mut self) -> SoundSample {

        let res = sample_and_remove(&mut self.synths) + sample_and_remove(&mut self.sub_mixers);

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
