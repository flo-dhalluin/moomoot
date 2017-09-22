use tree::bus;
use traits::ParamValue;

/// a parameter : this is something that's either a constant, or
/// a variable from the bus system.
pub enum SynthParam {
    /// constant set at instantiation
    Constant(f64),
    /// realtime value read from a bus
    BusValue(bus::Receiver<f64>),
    /// default value specified in implementation
    DefaultValue(f64),
}

impl SynthParam {
    pub fn value(&self) -> f64 {
        match *self {
            SynthParam::Constant(v) |
            SynthParam::DefaultValue(v) => v,
            SynthParam::BusValue(ref rcv) => rcv.value(),
        }
    }
}


pub trait SynthParams {
    /// returns the list of all available parameters
    fn list_params(&self) -> Vec<&str>;

    /// set one parameter
    fn set_param_value(&mut self, param_id: &str, value: SynthParam);
}

struct NullParameters;

impl SynthParams for NullParameters {
    fn list_params(&self) -> Vec<&str> {
        Vec::new()
    }
    fn set_param_value(&mut self, _: &str, _: SynthParam) {}
}

static mut NULL_PARAMETERS: NullParameters = NullParameters {};


pub trait Parametrized {
    fn get_params(&mut self) -> &mut SynthParams {

        unsafe {
            // whatever
            &mut NULL_PARAMETERS
        }
    }
}

pub fn init_parametrized<T: Parametrized + ?Sized>(
    synth: &mut T,
    buses: &mut bus::BusSystem,
    params: Vec<(String, ParamValue)>,
) {
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
