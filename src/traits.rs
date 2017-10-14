use std::ops::Add;
use std::iter::Sum;

#[derive(Debug, PartialEq)]
/// wraps actual sampled values : handle mono/stereo stuff
pub enum SampleValue {
    Stereo(f64, f64), // (left / right )
    Mono(f64),
}

/// auto mixer ! (note that mono are equally split l/r if needed)
impl Add for SampleValue {
    type Output = SampleValue;

    fn add(self, other: SampleValue) -> SampleValue {
        match self {
            SampleValue::Mono(x) => {
                match other {
                    SampleValue::Mono(y) => SampleValue::Mono(x + y),
                    SampleValue::Stereo(yr, yl) => SampleValue::Stereo(yr + x, yl + x),
                }
            }
            SampleValue::Stereo(xr, xl) => {
                match other {
                    SampleValue::Mono(y) => SampleValue::Stereo(xr + y, xl + y),
                    SampleValue::Stereo(yr, yl) => SampleValue::Stereo(yr + xr, yl + xr),
                }
            }
        }
    }
}

/// output for a synth either a sample, or silence
/// (done trigger autoremove from the tree )
#[derive(Debug, PartialEq)]
pub enum SoundSample {
    Sample(SampleValue),
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

pub fn mono_value(v: f64) -> SoundSample {
    SoundSample::Sample(SampleValue::Mono(v))
}

pub fn stereo_value(right: f64, left: f64) -> SoundSample {
    SoundSample::Sample(SampleValue::Stereo(right, left))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_automix_samples1() {

        let mono = mono_value(0.5);
        let silence = SoundSample::Silence;

        assert_eq!(SoundSample::Sample(SampleValue::Mono(0.5)), mono + silence);

    }

    #[test]
    fn test_automix_samples2() {

        let mono = mono_value(0.5);
        let stereo = stereo_value(0.2, 0.3);
        let silence = SoundSample::Silence;

        assert_eq!(SoundSample::Sample(SampleValue::Stereo(0.7, 0.8)), mono + stereo + silence);

    }


}
