pub mod noise;
pub mod string;
pub mod sine;

use traits::SoundSample;
use params::Parametrized;


pub trait Synth: Parametrized + Send {
    /// runs initializations (and gets the frame_t)
    fn init(&mut self, frame_t: f64) {}

    /// get one sound sample please
    fn sample(&mut self) -> SoundSample;
}
