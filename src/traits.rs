use std::ops::Add;
use std::iter::Sum;

/// output for a synth either a sample, or silence
/// (done trigger autoremove from the tree )
#[derive(Debug, PartialEq)]
pub enum SoundSample {
    Sample(f64),
    Silence, // no output
    Done, // this sound is done. kill me.
}

impl Add for SoundSample {
    type Output = SoundSample;

    // move add
    fn add(self, other: SoundSample) -> SoundSample {
        match self {
            SoundSample::Sample(x) => {
                match other {
                    SoundSample::Sample(y) => SoundSample::Sample(x + y),
                    _ => SoundSample::Sample(x),
                }
            }
            _ => other,
        }
    }
}

// that could be redefined for anything implementing Add and a
// neutral element ( group / monoid   ). Neutral is Done here.
impl Sum for SoundSample {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = SoundSample>,
    {
        iter.fold(SoundSample::Done, |a, b| a + b)
    }
}
