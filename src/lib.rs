mod base;
//mod looper;
pub mod synth;
pub mod instrument;

pub use base::MooMoot;
pub use base::Synth;
pub use base::SoundSample;

//pub use looper::Looper;

pub fn say_yo() {
    print!("yraco !");
}
