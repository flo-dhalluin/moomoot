pub mod noise;
pub mod string;
pub mod sine;

use traits::SoundSample;
use params::Parametrized;

/// Unit synthetiser trait.
pub trait Synth: Parametrized + Send {

    /// runs initializations (default implementation is noop)
    ///
    /// # Arguments
    /// * `frame_t` : the frame time (in seconds)
    fn init(&mut self, frame_t: f64) {}

    /// Generate one frame / sample
    ///
    /// This runs in the RealTime thread : no blocking !
    fn sample(&mut self) -> SoundSample;
}
