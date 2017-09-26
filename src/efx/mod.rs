pub mod volume;

use traits::SoundSample;
use params::Parametrized;

pub trait Efx: Parametrized {
    /// factory method : takes in the "frame time"
    fn init(&mut self, frame_t: f64) {}

    fn sample(&mut self, sample: f64) -> SoundSample;
}
