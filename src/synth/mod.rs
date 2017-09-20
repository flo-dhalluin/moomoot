pub mod noise;
pub mod string;
pub mod sine;

use traits::SoundSample;
use params::Parametrized;


pub trait Synth : Parametrized {

    /// factory method : takes in the "frame time"
    fn new(frame_t: f64) -> Self where Self:Sized;

    fn sample(&mut self) -> SoundSample;

}
