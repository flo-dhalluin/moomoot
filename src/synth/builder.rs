use super::Synth;
use super::SynthParams;
use super::SynthParam;
use traits::ParamValue;
use tree::bus;

pub fn init_synth(synth: &mut Synth, buses: &mut bus::BusSystem, params: Vec<(String, ParamValue)>) {
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
