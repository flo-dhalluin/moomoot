use super::mmtree;
use super::mixer;
use traits::*;

#[test]
fn create_tree() {

    let mut t = mmtree::MMTree::new();

    // always them root mixer
    assert_eq!(t.mixer_count(), 1);
    // structure.
    t.add_mixer("root", "mixer1");
    t.add_mixer("root", "mixer2");
    t.add_mixer("mixer1", "mixer11");

    assert_eq!(t.mixer_count(), 1 + 3, " there are 4 mixers");

}

// a synth that returns a constant.
struct CstSynth {
    value: f64
}

impl Synth for CstSynth {

    fn sample(&mut self) -> SoundSample {
        SoundSample::Sample(self.value)
    }
}

struct VolumeEfx {
    vol: f64
}

impl Efx for VolumeEfx {

    fn sample(&mut self, sample: f64) -> SoundSample {
        SoundSample::Sample( self.vol * sample)
    }
}

#[test]
fn mixer_sample() {

    let mut mixer = mixer::Mixer::new("test");

    mixer.add_synth(Box::new(CstSynth{value:1.}));
    mixer.add_synth(Box::new(CstSynth{value:3.}));


    assert_eq!(mixer.sample(), SoundSample::Sample(4.));
}

#[test]
fn mixer_efx() {
    let mut mixer = mixer::Mixer::new("test");
    mixer.add_synth(Box::new(CstSynth{value:1.}));
    mixer.add_efx(Box::new(VolumeEfx{vol:0.6}));

    assert_eq!(mixer.sample(), SoundSample::Sample(0.6));
}

#[test]
fn mixer_cascade() {

    let mut t = mmtree::MMTree::new();

    // always them root mixer
    assert_eq!(t.mixer_count(), 1);
    // structure.
    t.add_mixer("root", "mixer1");
    t.add_mixer("root", "mixer2");

    t.add_synth("mixer1", Box::new(CstSynth{value:0.1}));
    t.add_synth("mixer2", Box::new(CstSynth{value:0.3}));
    assert_eq!(t.sample(), SoundSample::Sample(0.4));

    t.add_efx("mixer2", Box::new(VolumeEfx{vol:0.5}));
    assert_eq!(t.sample(), SoundSample::Sample(0.25));
}

struct ShittyEnvelope {
    nb_samples: usize,
    tic: usize
}

impl ShittyEnvelope {
    fn new(nb_samples: usize) -> ShittyEnvelope {
        ShittyEnvelope {
            nb_samples: nb_samples,
            tic: 0
        }
    }
}

impl Efx for ShittyEnvelope {
    fn sample(&mut self, sample: f64) -> SoundSample {
        if self.tic < self.nb_samples {
            self.tic += 1;
            SoundSample::Sample(sample)
        } else {
            SoundSample::Done
        }
    }
}

#[test]
fn transient_mixers() {
    // transient mixers are mixers that get removed from the tree
    // when silenced .. (think the note of an instrument)

    let mut tree = mmtree::MMTree::new();

    let transient_mixer_id = tree.add_transient_mixer("root").unwrap();

    tree.add_synth(&transient_mixer_id, Box::new(CstSynth{value: 0.42}));
    tree.add_efx(&transient_mixer_id, Box::new(ShittyEnvelope::new(3)));

    tree.add_mixer("root", "not_transient");

    assert_eq!(tree.mixer_count(), 3);

    for i in 0..3 {
        assert_eq!(tree.sample(), SoundSample::Sample(0.42));
    }
    assert_eq!(tree.sample(), SoundSample::Silence);
    assert_eq!(tree.mixer_count(), 2);

}
