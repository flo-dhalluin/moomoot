use jack::prelude as j;
use std::sync::mpsc::*;
use uuid::Uuid;
use tree::mmtree::MMTree;
use traits::*;
use synth::Synth;
use efx::Efx;

type Params = Vec<(String, ParamValue)>;

// #[derive(Debug)]
enum InternalCmd {
    // internal commands to pass to RT thread
    AddMixer(String, String), // parent , kid
    AddEfx(String, Box<Efx>, Params),
    AddSynth(String, Box<Synth>, Params), // could be consumable iterator, instead of vec
    SetBusValue(String, f64),
}



// RT process callback
struct InternalProcess {
    out_port: j::Port<j::AudioOutSpec>,
    rx: Receiver<InternalCmd>,
    synth_tree: MMTree,
}

impl InternalProcess {
    // need lifetimes here so that we know that the borrow is released
    fn new(client: &j::Client) -> (Sender<InternalCmd>, InternalProcess) {
        let port = client
            .register_port("moomoot1", j::AudioOutSpec::default())
            .unwrap();
        let (sx, rx) = channel();
        let mut m = InternalProcess {
            out_port: port,
            rx: rx,
            synth_tree: MMTree::new(),
        };
        (sx, m)
    }

    fn command(&mut self, cmd: InternalCmd) -> Result<(), &str> {
        match cmd {
            InternalCmd::AddSynth(p, synth, params) => self.synth_tree.add_synth(&p, synth, params),
            InternalCmd::AddEfx(p, efx, params) => self.synth_tree.add_efx(&p, efx, params),
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
        let port = &mut self.out_port;
        let mut out_p = j::AudioOutPort::new(port, ps);
        let out: &mut [f32] = &mut out_p;

        // Write output
        for v in out.iter_mut() {
            match self.synth_tree.sample() {
                SoundSample::Sample(s) => {
                    *v = s as f32;
                }
                _ => {
                    *v = 0.;
                }
            }
        }

        // Continue as normal
        j::JackControl::Continue
    }
}

/// Opaque type for a mixer
pub struct MixerH(String);


/// The MooMooT server object.
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
    pub fn add_synth<T: 'static + Synth>(&mut self, mixer: &MixerH, params: Params) {
        let synth = Box::new(T::new(1. / self.sample_rate));
        self.send_channel
            .send(InternalCmd::AddSynth(mixer.0.clone(), synth, params))
            .expect("can't send command to MooMoot (RT process stopped)");
    }

    /// add an effect to them mixer
    pub fn add_efx<T: Efx + 'static>(&mut self, mixer: &MixerH, params: Params) {
        let efx = Box::new(T::new(1. / self.sample_rate));
        self.send_channel
            .send(InternalCmd::AddEfx(mixer.0.clone(), efx, params))
            .expect("can't send command to MooMoot (RT process stopped)");
    }

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
