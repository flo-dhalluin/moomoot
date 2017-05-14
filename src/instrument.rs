use super::base::Synth;


pub trait Instr {
    // this will alloc right ? 
    fn note(&self, fq: f64) -> Synth;
}
