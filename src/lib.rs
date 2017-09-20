extern crate uuid;
extern crate jack;

mod base;
mod traits;
pub mod synth;
pub mod efx;
mod tree;

pub use base::MooMoot;

pub use traits::SoundSample;
pub use traits::ParamValue;
pub use synth::Synth;
