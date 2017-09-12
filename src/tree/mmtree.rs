
use tree::mixer::Mixer;
use traits::*;
use uuid::Uuid;

pub struct MMTree {
    root_mixer: Box<Mixer>
}

impl MMTree {
    pub fn new() -> MMTree {
        MMTree { root_mixer: Box::new(Mixer::new("root"))}
    }


    // "transient" mixer, removed when AsMut
    pub fn add_transient_mixer(&mut self, parent: &str) -> Result<String, &'static str> {
        if let Some(mxr) = self.root_mixer.find_mixer(parent) {
            let uuid = Uuid::new_v4(); //Uuid::new_v4();
            let mixer_id = uuid.simple().to_string();
            mxr.add_sub_mixer( Mixer::new_transient(&mixer_id));
            Ok(mixer_id)
        } else {
            Err("can't find parent mixer")
        }
    }

    // "named" mixer. It'll stay there. CANNOT have a non named parent
    pub fn add_mixer(&mut self, parent: &str, mixer_id: &str) -> Result<(), &'static str> {

        if let Some(mxr) = self.root_mixer.find_mixer(parent) {
            mxr.add_sub_mixer( Mixer::new(mixer_id));
            Ok(())
        } else {
            Err("can't find parent mixer")
        }
    }

    // takes a Box, as Synth is a trait.
    pub fn add_synth(&mut self, mixer_id: &str, synth: Box<Synth>) -> Result<(), &'static str> {

        if let Some(mxr) = self.root_mixer.find_mixer(mixer_id) {
            mxr.add_synth(synth);
            Ok(())
        } else {
            Err("can't find parent mixer")
        }
    }

    pub fn add_efx(&mut self, mixer_id: &str, fx: Box<Efx>) -> Result<(), &'static str> {

        if let Some(mxr) = self.root_mixer.find_mixer(mixer_id) {
            mxr.add_efx(fx);
            Ok(())
        } else {
            Err("can't find parent mixer")
        }
    }


    pub fn mixer_count(&self) -> usize {
        return self.root_mixer.mixer_count();
    }

    pub fn sample(&mut self) -> SoundSample {
        return self.root_mixer.sample();
    }

}
