pub mod noise;
pub mod string;
pub mod sine;
pub mod builder;

use traits::*;
use tree::bus;

pub enum SynthParam {
    Constant(f64),
    BusValue(bus::Receiver<f64>),
    DefaultValue(f64)
}



pub trait SynthParams {

    fn list_params(&self) -> Vec<&str> {
        Vec::new()
    }
    fn set_param_value(&mut self, param_id: &str, value: SynthParam) {
    }
}

struct NullParameters;

impl SynthParams for NullParameters {
}

static mut NULL_PARAMETERS: NullParameters = NullParameters{};

pub trait Synth {

    /// factory method : takes in the "frame time"
    fn new(frame_t: f64) -> Self where Self:Sized;

    fn sample(&mut self) -> SoundSample;

    fn get_params(&mut self) -> &mut SynthParams {
        unsafe { // whatever
            &mut NULL_PARAMETERS
        }
    }
}


impl SynthParam {
    pub fn value(&self) -> f64 {
        match *self {
            SynthParam::Constant(v) | SynthParam::DefaultValue(v) => v,
            SynthParam::BusValue(ref rcv) => rcv.value()
        }
    }
}
