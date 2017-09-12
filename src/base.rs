extern crate jack;
use self::jack::prelude as j;
use std::sync::mpsc::*;

use tree::mmtree::MMTree;
use traits::*;

// #[derive(Debug)]
enum InternalCmd {
    // internal commands to pass to RT thread
    AddMixer(String, String),  // parent , kid
    AddEfx(String, Box<Efx>),
    AddSynth(String, Box<Synth>)
}



// RT process callback
struct InternalProcess {
    out_port : j::Port<j::AudioOutSpec>,
    rx : Receiver<InternalCmd>,
    synth_tree : MMTree
}

impl InternalProcess {

    // need lifetimes here so that we know that the borrow is released
    fn new(client: &j::Client) -> (Sender<InternalCmd>, InternalProcess) {
        let port = client.register_port("moomoot1", j::AudioOutSpec::default()).unwrap();
        let (sx, rx) = channel();
        let mut m = InternalProcess {
            out_port: port,
            rx: rx,
            synth_tree: MMTree::new()
        };
        (sx, m)
    }

    fn command(&mut self, cmd: InternalCmd) -> Result<(),&'static str>{
        match cmd {
            InternalCmd::AddSynth(p, synth) => self.synth_tree.add_synth(&p, synth),
            InternalCmd::AddEfx(p, efx) => self.synth_tree.add_efx(&p, efx),
            InternalCmd::AddMixer(p, mixer_id) => self.synth_tree.add_mixer(&p, &mixer_id)
        }
    }
}

impl j::ProcessHandler for InternalProcess {
    // realtime callback
    fn process(&mut self, _:&j::Client, ps: &j::ProcessScope) -> j::JackControl {

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
                SoundSample::Sample(s) => {*v = s as f32;}
                _ => {*v = 0.;}
            }
        }

        // Continue as normal
        j::JackControl::Continue
    }
}


pub struct MooMoot {
    async_client : j::AsyncClient<(), InternalProcess>,
    sample_rate : f64,
    send_channel : Sender<InternalCmd>
}

impl MooMoot {
    pub fn start() -> MooMoot {
            // 1. open a client
            let (client, _status) = j::Client::new("MooMoot", j::client_options::NO_START_SERVER).unwrap();
            let sample_rate = client.sample_rate() as f64;

            let (cmd_chan, process) = InternalProcess::new( &client);
            // 4. activate the client
            let active_client = j::AsyncClient::new(client, (), process).unwrap();

            MooMoot { async_client: active_client,
                sample_rate: sample_rate,
                 send_channel:cmd_chan}
    }

    // they shall error ?
    pub fn add_mixer(&mut self, parent: &str, kid: &str) {
        self.send_channel
            .send(InternalCmd::AddMixer(parent.to_string(), kid.to_string()))
            .expect("can't send command to MooMoot (RT process stopped)");
    }

    pub fn add_synth<T:Synth + 'static>(&mut self, parent: &str, synth: T) {
        self.send_channel
            .send(InternalCmd::AddSynth(parent.to_string(), Box::new(synth)))
            .expect("can't send command to MooMoot (RT process stopped)");
    }

    pub fn add_efx<T:Efx + 'static>(&mut self, parent: &str, efx: T) {
        self.send_channel
            .send(InternalCmd::AddEfx(parent.to_string(), Box::new(efx)))
            .expect("can't send command to MooMoot (RT process stopped)");
    }

    pub fn get_sampling_rate(&self) -> f64 {
        self.sample_rate
    }

}
