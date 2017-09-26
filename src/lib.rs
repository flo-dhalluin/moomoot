extern crate uuid;
extern crate jack;

mod base;
mod traits;
#[macro_use]
mod params;
pub mod synth;
pub mod efx;
mod tree;

pub use base::MooMoot;

pub use traits::SoundSample;
pub use params::ParamValue;
pub use synth::Synth;
