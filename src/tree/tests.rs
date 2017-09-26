use super::mmtree;
use super::mixer;
use super::bus::{Bus, Receiver, BusSystem};
use traits::*;
use params::*;
use synth::Synth;
use efx::Efx;
use efx::volume::{Volume, VolumeParams};

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
    value: f64,
}

impl CstSynth {
    fn new(v: f64) -> CstSynth{
        CstSynth{value: v}
    }
}

impl Synth for CstSynth {

    fn sample(&mut self) -> SoundSample {
        SoundSample::Sample(self.value)
    }
}

impl Parametrized for CstSynth {}


#[test]
fn mixer_sample() {

    let mut mixer = mixer::Mixer::new("test");

    mixer.add_synth(Box::new(CstSynth::new(1.0)));
    mixer.add_synth(Box::new(CstSynth::new(3.0)));


    assert_eq!(mixer.sample(), SoundSample::Sample(4.));
}

#[test]
fn mixer_efx() {
    let mut mixer = mixer::Mixer::new("test");
    mixer.add_synth(Box::new(CstSynth::new(1.0)));
    let mut v = Box::new(Volume::new(VolumeParams::default().volume(0.6)));
    mixer.add_efx(v);

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

    t.add_synth("mixer1", Box::new(CstSynth::new(0.1)));
    t.add_synth("mixer2", Box::new(CstSynth::new(0.3)));
    assert_eq!(t.sample(), SoundSample::Sample(0.4));

    t.add_efx(
        "mixer2",
        Box::new(Volume::new(VolumeParams::default().volume(0.5)))
    ); // 0.5
    assert_eq!(t.sample(), SoundSample::Sample(0.25));
}

struct ShittyEnvelope {
    nb_samples: usize,
    tic: usize,
}

impl Parametrized for ShittyEnvelope {}

impl ShittyEnvelope {
    fn new() -> ShittyEnvelope {
        ShittyEnvelope {
            nb_samples: 3,
            tic: 0,
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

    tree.add_synth(
        &transient_mixer_id,
        Box::new(CstSynth::new(0.42))
    );
    tree.add_efx(
        &transient_mixer_id,
        Box::new(ShittyEnvelope::new())
    );

    tree.add_mixer("root", "not_transient");

    assert_eq!(tree.mixer_count(), 3);

    for i in 0..3 {
        assert_eq!(tree.sample(), SoundSample::Sample(0.42));
    }
    assert_eq!(tree.sample(), SoundSample::Silence);
    assert_eq!(tree.mixer_count(), 2);
}

struct ThemSubsc {
    r: Receiver<f64>,
}


impl ThemSubsc {
    fn new(bus: &mut Bus<f64>) -> ThemSubsc {
        ThemSubsc { r: bus.sub() }
    }

    fn yo(&self) -> f64 {
        self.r.value()
    }
}

#[test]
fn test_bus() {
    let mut bus = Bus::new("bus", 0.0);
    let tsuone = ThemSubsc::new(&mut bus);
    {
        let tsu = ThemSubsc::new(&mut bus);
        assert_eq!(tsu.yo(), 0.0);
        bus.publish(42.0);
        assert_eq!(tsu.yo(), 42.0);
        assert_eq!(tsuone.yo(), 42.0);
        assert_eq!(bus.subscriber_count(), 2);
    }
    bus.publish(2.);
    assert_eq!(tsuone.yo(), 2.0);
    assert_eq!(bus.subscriber_count(), 1);
}

struct Stuff {
    a: Receiver<f64>,
    b: Receiver<f64>,
}


impl Stuff {
    fn doit(&self) -> f64 {
        self.a.value() + self.b.value()
    }
}

#[test]
fn test_bus_system() {

    let mut bus = BusSystem::new();

    let stuff = Stuff {
        a: bus.sub("a"),
        b: bus.sub("b"),
    };

    bus.publish("a", 2.0);
    bus.publish("b", 4.0);

    assert_eq!(stuff.doit(), 6.0);

    assert!(bus.publish("b", 5.0).is_ok());
    assert!(bus.publish("d", 5.0).is_err());
}

declare_params!(CstSynthParams {value: 1.});

struct CstSynthWithP {
    params: CstSynthParams,
}

impl CstSynthWithP {
    fn new(params: CstSynthParams) -> CstSynthWithP {
        CstSynthWithP{params: params}
    }
}

impl Parametrized for CstSynthWithP {
    fn get_parameters(&mut self) -> &mut Parameters {
        &mut self.params
    }
}

impl Synth for CstSynthWithP {
    fn sample(&mut self) -> SoundSample {
        SoundSample::Sample(self.params.value.value())
    }
}

#[test]
fn synth_with_params() {
    let mut tree = mmtree::MMTree::new();
    tree.add_synth("root", Box::new(CstSynthWithP::new(CstSynthParams::default())));

    assert_eq!(tree.sample(), SoundSample::Sample(1.));

    tree.add_synth(
        "root",
        Box::new(CstSynthWithP::new(CstSynthParams::default().value(0.44))));

    assert_eq!(tree.sample(), SoundSample::Sample(1.44));

    tree.add_synth(
        "root",
        Box::new(CstSynthWithP::new(CstSynthParams::default().value("chombier"))));

    tree.set_bus_value("chombier", 1.0);
    assert_eq!(tree.sample(), SoundSample::Sample(2.44));
    tree.set_bus_value("chombier", 0.33);
    assert_eq!(tree.sample(), SoundSample::Sample(1.77));


}
