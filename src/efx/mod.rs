pub mod volume;
use synth::Parametrized;
use traits::SoundSample;

pub trait Efx : Parametrized {

    /// factory method : takes in the "frame time"
    fn new(frame_t: f64) -> Self where Self:Sized;

    fn sample(&mut self, sample: f64) -> SoundSample;
}
