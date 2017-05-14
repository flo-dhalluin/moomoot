extern crate jack;
use self::jack::prelude::{AudioOutPort, AudioOutSpec, Client, JackControl, ProcessHandler,
                    ProcessScope, Port};

pub trait Synth {
    fn reset(&mut self);
    fn sample(&mut self, frame_t: f64) -> f64;
}

pub struct MooMoot <T:Synth > {
    out_port : Port<AudioOutSpec>,
    frame_t: f64,
    synth: T,
}

impl<T:Synth> MooMoot<T> {
    pub fn new(client: &Client, synth: T) -> MooMoot<T> {
        let port = client.register_port("sine_out", AudioOutSpec::default()).unwrap();
        let sample_rate = client.sample_rate();
        MooMoot {
            out_port: port,
            frame_t: 1.0 / sample_rate as f64,
            synth: synth,
        }
    }
}

impl<T:Synth> ProcessHandler for MooMoot<T> {
    fn process(&mut self, _:&Client, ps: &ProcessScope) -> JackControl {
        // Get output buffer
        let mut out_p = AudioOutPort::new(&mut self.out_port, ps);
        let out: &mut [f32] = &mut out_p;

        // Check frequency requests
        /*while let Ok(f) = rx.try_recv() {
            time = 0.0;
            frequency = f;
        }*/

        // Write output
        for v in out.iter_mut() {
            *v = self.synth.sample(self.frame_t) as f32;
        }

        // Continue as normal
        JackControl::Continue
    }
}
