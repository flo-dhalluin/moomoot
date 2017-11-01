use jack::prelude as j;
use std::sync::mpsc::*;
use uuid::Uuid;
use tree::mmtree::MMTree;
use traits::*;
use synth::Synth;
use efx::Efx;


// #[derive(Debug)]
enum InternalCmd {
    // internal commands to pass to RT thread
    AddMixer(String, String), // parent , kid
    AddEfx(String, Box<Efx>),
    AddSynth(String, Box<Synth>),
    SetBusValue(String, f64),
}



// RT process callback
struct InternalProcess {
    ports: (j::Port<j::AudioOutSpec>, j::Port<j::AudioOutSpec>), // (R,L) jack ports
    rx: Receiver<InternalCmd>,
    synth_tree: MMTree,
}

impl InternalProcess {
    // need lifetimes here so that we know that the borrow is released
    fn new(client: &j::Client) -> (Sender<InternalCmd>, InternalProcess) {
        let port_right = client
            .register_port("moomoot_r", j::AudioOutSpec::default())
            .unwrap();
        let port_left = client
            .register_port("moomoot_l", j::AudioOutSpec::default())
            .unwrap();

        let (sx, rx) = channel();
        let m = InternalProcess {
            ports: (port_right, port_left),
            rx: rx,
            synth_tree: MMTree::new(),
        };
        (sx, m)
    }

    fn command(&mut self, cmd: InternalCmd) -> Result<(), &str> {
        match cmd {
            InternalCmd::AddSynth(p, synth) => self.synth_tree.add_synth(&p, synth),
            InternalCmd::AddEfx(p, efx) => self.synth_tree.add_efx(&p, efx),
            InternalCmd::AddMixer(p, mixer_id) => self.synth_tree.add_mixer(&p, &mixer_id),
            InternalCmd::SetBusValue(bus, value) => self.synth_tree.set_bus_value(&bus, value),
        }
    }
}

impl j::ProcessHandler for InternalProcess {
    // realtime callback
    fn process(&mut self, _: &j::Client, ps: &j::ProcessScope) -> j::JackControl {

        // treat command buffer
        while let Ok(msg) = self.rx.try_recv() {
            self.command(msg).expect("panic in synth tree:");
        }

        // Get output buffer
        let mut out_right = j::AudioOutPort::new(&mut self.ports.0, ps);
        let mut out_left = j::AudioOutPort::new(&mut self.ports.1, ps);

        let out_r: &mut [f32] = &mut out_right;
        let out_l: &mut [f32] = &mut out_left;

        // Write output
        for (v_r, v_l) in out_r.iter_mut().zip(out_l.iter_mut()) {
            match self.synth_tree.sample() {
                SoundSample::Sample(sample) => {
                    match sample {
                        SampleValue::Mono(s) => *v_r = s as f32,
                        SampleValue::Stereo(r, l) => {
                            *v_r = r as f32;
                            *v_l = l as f32;
                        }
                    }
                }
                _ => {
                    *v_r = 0.;
                    *v_l = 0.;
                }
            }
        }

        // Continue as normal
        j::JackControl::Continue
    }
}

/// Opaque type for a mixer
pub struct MixerH(String);


/// The MooMooT Synthetizer object.
/// # Example
/// ```
/// use moomoot::synth::sine::{Sine, SineParams};
///  // requires a running jack daemon
/// let mut moomoot = moomoot::MooMoot::start();
/// let root_mixer = moomoot.root_mixer();
///  // add a 440 Hz perfect sine to the root mixer.
/// moomoot.add_synth(&root_mixer, Sine::new(SineParams::default().frequency(440.0)));
/// ```
pub struct MooMoot {
    async_client: j::AsyncClient<(), InternalProcess>,
    sample_rate: f64,
    send_channel: Sender<InternalCmd>,
}

impl MooMoot {
    /// Create a MooMooT instance, instantiate the jack port
    /// and starts jack RT thread.
    pub fn start() -> MooMoot {
        // 1. open a client
        let (client, _status) = j::Client::new("MooMoot", j::client_options::NO_START_SERVER)
            .unwrap();
        let sample_rate = client.sample_rate() as f64;

        let (cmd_chan, process) = InternalProcess::new(&client);
        // 4. activate the client
        let active_client = j::AsyncClient::new(client, (), process).unwrap();

        MooMoot {
            async_client: active_client,
            sample_rate: sample_rate,
            send_channel: cmd_chan,
        }
    }
    /// Disconnect from Jack
    pub fn kill(self) {
        self.async_client.deactivate().unwrap();
    }

    /// get an handle to the "root" mixer
    pub fn root_mixer(&self) -> MixerH {
        MixerH("root".to_string())
    }

    /// create a mixer node.
    pub fn add_mixer(&mut self, parent: &MixerH, name_prefix: &str) -> MixerH {

        let mixer_id = format!("{}-{}", name_prefix, Uuid::new_v4().simple());
        self.send_channel
            .send(InternalCmd::AddMixer(parent.0.clone(), mixer_id.clone()))
            .expect("can't send command to MooMoot (RT process stopped)");
        MixerH(mixer_id)
    }

    /// add a synth to a mixer node.
    pub fn add_synth<T: 'static + Synth>(&mut self, mixer: &MixerH, mut synth: T) {
        synth.init(1. / self.sample_rate);
        self.send_channel
            .send(InternalCmd::AddSynth(mixer.0.clone(), Box::new(synth)))
            .expect("can't send command to MooMoot (RT process stopped)");
    }

    /// add an effect to a mixer
    pub fn add_efx<T: Efx + 'static>(&mut self, mixer: &MixerH, mut efx: T) {
        efx.init(1. / self.sample_rate);
        self.send_channel
            .send(InternalCmd::AddEfx(mixer.0.clone(), Box::new(efx)))
            .expect("can't send command to MooMoot (RT process stopped)");
    }

    /// set a new parameter value in the bus system
    pub fn set_bus_value(&mut self, bus: &str, value: f64) {
        self.send_channel
            .send(InternalCmd::SetBusValue(bus.to_string(), value))
            .expect("can't send command to MooMoot (RT process stopped)");
    }

    /// returns sampling rate ( in seconds )
    pub fn get_sampling_rate(&self) -> f64 {
        self.sample_rate
    }
}
