
use tree::mixer::Mixer;
use traits::*;
use super::pbus::BusSystem;
use synth::Synth;
use efx::Efx;
use uuid::Uuid;

pub struct MMTree {
    root_mixer: Box<Mixer>,
    buses: BusSystem,
}


impl MMTree {
    pub fn new() -> MMTree {
        MMTree {
            root_mixer: Box::new(Mixer::new("root")),
            buses: BusSystem::new(),
        }
    }


    // "transient" mixer, removed when AsMut
    pub fn add_transient_mixer(&mut self, parent: &str) -> Result<String, &str> {
        if let Some(mxr) = self.root_mixer.find_mixer(parent) {
            let uuid = Uuid::new_v4(); //Uuid::new_v4();
            let mixer_id = uuid.simple().to_string();
            mxr.add_sub_mixer(Mixer::new_transient(&mixer_id));
            Ok(mixer_id)
        } else {
            Err("can't find parent mixer")
        }
    }

    // "named" mixer. It'll stay there. CANNOT have a non named parent
    pub fn add_mixer(&mut self, parent: &str, mixer_id: &str) -> Result<(), &str> {

        if let Some(mxr) = self.root_mixer.find_mixer(parent) {
            mxr.add_sub_mixer(Mixer::new(mixer_id));
            Ok(())
        } else {
            Err("can't find parent mixer")
        }
    }

    // takes a Box, as Synth is a trait.
    pub fn add_synth(&mut self, mixer_id: &str, mut synth: Box<Synth>) -> Result<(), &str> {

        synth.as_mut().connect_parameters(&mut self.buses);

        if let Some(mxr) = self.root_mixer.find_mixer(mixer_id) {
            mxr.add_synth(synth);
            Ok(())
        } else {
            Err("can't find parent mixer")
        }
    }

    pub fn set_bus_value(&mut self, bus: &str, value: f64) -> Result<(), &str> {
        self.buses.publish(bus, value).map_err(
            |_| "no such channel",
        )
    }

    pub fn add_efx(&mut self, mixer_id: &str, mut fx: Box<Efx>) -> Result<(), &str> {

        fx.as_mut().connect_parameters(&mut self.buses);

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
