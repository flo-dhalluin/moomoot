use tree::bus;
use traits::ParamValue;
/// a parameter : this is something that's either a constant, or
/// a variable from the bus system.
pub enum SynthParam {
    Constant(f64),
    BusValue(bus::Receiver<f64>),
    DefaultValue(f64)
}

impl SynthParam {
    pub fn value(&self) -> f64 {
        match *self {
            SynthParam::Constant(v) | SynthParam::DefaultValue(v) => v,
            SynthParam::BusValue(ref rcv) => rcv.value()
        }
    }
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

pub trait Parametrized {
    fn get_params(&mut self) -> &mut SynthParams {
        unsafe { // whatever
            &mut NULL_PARAMETERS
        }
    }
}

pub fn init_parametrized<T : Parametrized + ?Sized>(synth: &mut T, buses: &mut bus::BusSystem, params: Vec<(String, ParamValue)>) {
    let mut sparams = synth.get_params();
    for (name, value) in params.into_iter() {
        match value {
            ParamValue::Constant(v) => sparams.set_param_value(&name, SynthParam::Constant(v)),
            ParamValue::BusValue(bus_name) => {
                let rcv = buses.sub(&bus_name);
                sparams.set_param_value(&name, SynthParam::BusValue(rcv));
            }
        }
    }
}
