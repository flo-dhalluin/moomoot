pub mod volume;
pub mod pan;
use traits::{SampleValue, SoundSample};
use params::Parametrized;

pub trait Efx: Parametrized + Send {
    /// factory method : takes in the "frame time"
    fn init(&mut self, frame_t: f64) {}

    fn sample(&mut self, smpl : SampleValue) -> SoundSample;
}
