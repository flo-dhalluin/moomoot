use std::mem;

/// a non thread safe fixed size ring buffer
pub struct FixedRingBuffer {
    queue: Box<[f64]>,
    idx: usize, // index of last input ( so output is right behind)
}


impl FixedRingBuffer {
    #[inline]
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// queue/dequeue in one op.
    pub fn queue(&mut self, elem: &mut f64) {
        let len = self.len();
        self.idx = (self.idx + len - 1) % len;
        //println!(" queue idx : {} / {} <= {}", self.idx, len, elem);
        mem::swap(unsafe { self.queue.get_unchecked_mut(self.idx) }, elem);
    }

    /// reset all elements
    pub fn set_all(&mut self, elem: f64) {
        for e in self.queue.iter_mut() {
            *e = elem;
        }
    }
}

impl From<Vec<f64>> for FixedRingBuffer {
    fn from(vec: Vec<f64>) -> Self {
        FixedRingBuffer {
            queue: vec.into_boxed_slice(),
            idx: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ringbuffer() {

        let mut ring = FixedRingBuffer::from(vec![0.0; 10]);

        for i in 1..100 {
            let mut value = i as f64;
            ring.queue(&mut value);
            if i < 10 {
                assert_eq!(value, 0.0);
            } else {
                assert_eq!(value, (i as f64) - 10.0);
            }

        }
    }
}
