use std::ops::{Add, AddAssign};
use std::iter::Sum;

#[derive(Debug, PartialEq, Clone)]
/// A non silence sound frame. could be Mono, or Stereo
///
/// MooMooT sample are *always* f64 (even if the underlying JACK engine takes floats)
pub enum SampleValue {
    /// Stereo (right , left)
    Stereo(f64, f64),
    /// Mono
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

impl AddAssign for SampleValue {
    fn add_assign(&mut self, other: SampleValue) {
        match other {
            SampleValue::Mono( mono ) => {
                match *self {
                    SampleValue::Mono(ref mut y) => *y += mono,
                    SampleValue::Stereo(ref mut l, ref mut r) => {
                        *l += mono;
                        *r += mono;
                    },
                }
            },
            SampleValue::Stereo( left, right) => {
                match *self {
                    SampleValue::Mono(mono) => {
                        *self = SampleValue::Stereo(left + mono,
                        right + mono);
                    },
                    SampleValue::Stereo(ref mut sleft,
                        ref mut sright) => {
                        *sleft += left;
                        *sright += right;
                    },
                }
            }
        }
    }
}

/// A frame value. Can be an actual sound frame, or silence
///
/// There is two kind of silence : `Silence` and `Done`. When a `Synth` or `Efx`
/// Outputs `Done` it is signal that it should be removed from the synthesis tree
#[derive(Debug, PartialEq)]
pub enum SoundSample {
    /// A sound frame
    Sample(SampleValue),
    /// Silence
    Silence,
    /// Flag for removal
    Done,
}

/// Sample are summable for mixers.
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

// when you don't feel like consuming
impl AddAssign for SoundSample {

    fn add_assign(&mut self, other: SoundSample) {
        if let SoundSample::Sample(x) = other {
            match self {
                SoundSample::Sample(ref mut sx) => {
                    *sx += x;
                },
                _ => { *self = SoundSample::Sample(x); },
            }
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

/// Creates a Mono sound frame
pub fn mono_value(v: f64) -> SoundSample {
    SoundSample::Sample(SampleValue::Mono(v))
}

/// Create a stereo sound frame
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

    #[test]
    fn test_automix_samples3() {

        let mut sample = mono_value(0.5);
        sample += stereo_value(0.2, 0.3);
        sample += SoundSample::Silence;

        assert_eq!(SoundSample::Sample(SampleValue::Stereo(0.7, 0.8)), sample);

    }


}
