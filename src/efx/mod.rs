pub mod volume;
pub mod pan;
use traits::{SampleValue, SoundSample};
use params::Parametrized;

/// Effect to add to a Mixer
pub trait Efx: Parametrized + Send {
    /// initialize the effect
    ///
    /// #  takes in the "frame time"
    #[allow(unused_variables)]
    fn init(&mut self, frame_t: f64) {}

    fn sample(&mut self, smpl: SampleValue) -> SoundSample;
}
