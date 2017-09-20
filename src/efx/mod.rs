pub mod volume;

use traits::SoundSample;
use params::Parametrized;

pub trait Efx : Parametrized {

    /// factory method : takes in the "frame time"
    fn new(frame_t: f64) -> Self where Self:Sized;
    fn sample(&mut self, sample: f64) -> SoundSample;

}
