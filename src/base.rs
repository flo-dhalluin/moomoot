extern crate jack;
use self::jack::prelude as j;
use std::collections::HashMap;
use std::sync::mpsc::*;

use synth::string::KarplusStrong;
use synth::sine::Sine;

/** output for a synth either a sample, or silence (done trigger autoremove from the tree )*/
pub enum SoundSample {
    Sample(f64),
    Done,
}


pub trait Synth {
    fn sample(&mut self) -> SoundSample;
}

pub trait Efx {
    fn sample(&mut self, sample: f64) -> SoundSample;
}

// combinators

struct Mixer {
    synths: Vec<Box<Synth>>
}

impl Mixer {

    pub fn new() -> Mixer {
        Mixer { synths: Vec::new() }
    }

    pub fn add_synth(&mut self, s: Box<Synth>) {
        self.synths.push(s);
    }
}

impl Synth for Mixer {
    fn sample(&mut self) -> SoundSample {
        let mut result:f64 = 0.;
        let mut actives = 0;
        let mut v: Vec<usize> = Vec::new();
        for (idx, s) in self.synths.iter_mut().enumerate() {
            match s.sample() {
                SoundSample::Sample(x) => { result += x; actives += 1 },
                SoundSample::Done => {v.push(idx);}
            }
        }
        for idx in v {
            self.synths.remove(idx);
        }

        if(actives > 0 ) {
            return SoundSample::Sample(result);
        }
        SoundSample::Done
    }
}

pub enum MooMootCmd {
    AddSynth(String),
    DelSynth(String)
}

type SynthMaker = Fn(f64) -> Box<Synth>;

// RT process callback
struct MooMootProcess {
    out_port : j::Port<j::AudioOutSpec>,
    root_mixer : Mixer,
    sample_rate : f64,
    rx : Receiver<MooMootCmd>,
    synth_makers: HashMap<String, Box<SynthMaker>>,
}

impl MooMootProcess {

    // need lifetimes here so that we know that the borrow is released
    fn new(client: &j::Client) -> (Sender<MooMootCmd>, MooMootProcess) {
        let port = client.register_port("moomoot1", j::AudioOutSpec::default()).unwrap();
        let sample_rate = client.sample_rate();
        let (sx, rx) = channel();
        let mut m = MooMootProcess {
            out_port: port,
            sample_rate: sample_rate as f64,
            root_mixer: Mixer::new(),
            synth_makers: HashMap::new(),
            rx: rx
        };

        let kps = Box::new(|srate: f64| -> Box<Synth> {
            Box::new(KarplusStrong::new(220., 1./srate, 6000., 0.99))
        });
        m.synth_makers.insert(String::from("kps"), kps);

        let sine_m = Box::new(|srate: f64| -> Box<Synth> {
            Box::new(Sine::new(srate, 440.))
        });

        m.synth_makers.insert(String::from("sine"), sine_m);

        (sx, m)
    }

    fn command(&mut self, cmd: MooMootCmd) {
        match cmd {
            MooMootCmd::AddSynth(name) => {
                if let Some(fact) = self.synth_makers.get(&name) {
                    self.root_mixer.add_synth(fact(self.sample_rate));
                } else {
                    println!("no such synth!");
                }
            }
            MooMootCmd::DelSynth(_) => {
                println!("Unsupported command!");
            }
        }
    }
}

impl j::ProcessHandler for MooMootProcess {
    // realtime callback
    fn process(&mut self, _:&j::Client, ps: &j::ProcessScope) -> j::JackControl {

        // treat command buffer
        while let Ok(msg) = self.rx.try_recv() {
            self.command(msg);
        }

        // Get output buffer
        let port = &mut self.out_port;
        let mut out_p = j::AudioOutPort::new(port, ps);
        let out: &mut [f32] = &mut out_p;

        // Check frequency requests
        /*while let Ok(f) = rx.try_recv() {
            time = 0.0;
            frequency = f;
        }*/

        // Write output
        for v in out.iter_mut() {
            match self.root_mixer.sample() {
                SoundSample::Sample(s) => {*v = s as f32;}
                SoundSample::Done => {*v = 0.;}
            }
        }

        // Continue as normal
        j::JackControl::Continue
    }
}


pub struct MooMoot {
    async_client : j::AsyncClient<(), MooMootProcess>,
}

impl MooMoot {
    pub fn start() -> (Sender<MooMootCmd>, MooMoot) {
            // 1. open a client
            let (client, _status) = j::Client::new("MooMoot", j::client_options::NO_START_SERVER).unwrap();
            let (cmd_chan, process) = MooMootProcess::new( &client);
            // 4. activate the client
            let active_client = j::AsyncClient::new(client, (), process).unwrap();

            (cmd_chan, MooMoot { async_client: active_client})
    }
}
