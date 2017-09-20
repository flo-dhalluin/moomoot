use super::mmtree;
use super::mixer;
use super::bus::{Bus,Receiver, BusSystem};
use traits::*;
use synth::{Synth, SynthParam, SynthParams, Parametrized};
use efx::Efx;
use efx::volume::Volume;

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

    fn new(_:f64) -> CstSynth {
        CstSynth{value: 42.0}
    }

    fn sample(&mut self) -> SoundSample {
        SoundSample::Sample(self.value)
    }
}

impl Parametrized for CstSynth {}


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
    mixer.add_synth(Box::new(CstSynth{value: 1.0}));
    let mut v = Box::new(Volume::new(1.0));
    v.as_mut().get_params().set_param_value("volume", SynthParam::Constant(0.6));
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

    t.add_synth("mixer1", Box::new(CstSynth{value:0.1}), Vec::new());
    t.add_synth("mixer2", Box::new(CstSynth{value:0.3}), Vec::new());
    assert_eq!(t.sample(), SoundSample::Sample(0.4));

    t.add_efx("mixer2", Box::new(Volume::new(1.0)), vec![("volume".to_string(), ParamValue::Constant(0.5))]); // 0.5
    assert_eq!(t.sample(), SoundSample::Sample(0.25));
}

struct ShittyEnvelope {
    nb_samples: usize,
    tic: usize
}

impl Parametrized for ShittyEnvelope {}


impl Efx for ShittyEnvelope {

    fn new(_ : f64) -> ShittyEnvelope {
        ShittyEnvelope {
            nb_samples: 3,
            tic: 0
        }
    }


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

    tree.add_synth(&transient_mixer_id, Box::new(CstSynth{value: 0.42}), Vec::new());
    tree.add_efx(&transient_mixer_id, Box::new(ShittyEnvelope::new(42.00)), Vec::new());

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
    b: Receiver<f64>
}


impl Stuff {
    fn doit(&self) -> f64 {
        self.a.value() + self.b.value()
    }
}

#[test]
fn test_bus_system() {

    let mut bus = BusSystem::new();

    let stuff = Stuff{a:bus.sub("a"), b:bus.sub("b")};

    bus.publish("a", 2.0);
    bus.publish("b", 4.0);

    assert_eq!(stuff.doit(), 6.0);

    assert!(bus.publish("b", 5.0).is_ok());
    assert!(bus.publish("d", 5.0).is_err());
}

struct CstSynthParams {
    value: SynthParam
}

impl SynthParams for CstSynthParams {
    fn list_params(&self) -> Vec<&str> {
        vec!["value"]
    }

    fn set_param_value(&mut self, param_id:&str, value: SynthParam) {
        match param_id {
            "value" => self.value = value,
            _ => {}
        }
    }
}

struct CstSynthWithP {
    params: CstSynthParams
}

impl Parametrized for CstSynthWithP {
    fn get_params(&mut self) -> &mut SynthParams {
        &mut self.params
    }

}

impl Synth for CstSynthWithP {
    fn new(frame_t:f64) -> CstSynthWithP {
        CstSynthWithP{params:CstSynthParams{value:SynthParam::DefaultValue(1.)}}
    }

    fn sample(&mut self) -> SoundSample {
        SoundSample::Sample(self.params.value.value())
    }

}

#[test]
fn synth_with_params() {
    let mut tree = mmtree::MMTree::new();
    tree.add_synth("root", Box::new(CstSynthWithP::new(0.)), Vec::new());
    assert_eq!(tree.sample(), SoundSample::Sample(1.));

    tree.add_synth("root", Box::new(CstSynthWithP::new(0.)), vec![("value".to_string(), ParamValue::Constant(0.44))]);
    assert_eq!(tree.sample(), SoundSample::Sample(1.44));

    tree.add_synth("root", Box::new(CstSynthWithP::new(0.)), vec![("value".to_string(), ParamValue::BusValue("chombier".to_string()))]);
    tree.set_bus_value("chombier", 1.0);
    assert_eq!(tree.sample(), SoundSample::Sample(2.44));
    tree.set_bus_value("chombier", 0.33);
    assert_eq!(tree.sample(), SoundSample::Sample(1.77));


}
