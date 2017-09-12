extern crate uuid;

mod base;
mod traits;
pub mod synth;
pub mod efx;
mod tree;

pub use base::MooMoot;

pub use traits::SoundSample;
pub use traits::Synth;

//pub use looper::Looper;

pub fn say_yo() {
    print!("yraco !");
}
