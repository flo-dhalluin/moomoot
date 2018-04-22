use std::sync::{Arc, Weak};
use std::sync::atomic;
use std::cell::UnsafeCell;

/// internal cell you can atomically write to
struct ReceiveCell<T> {
    v1: UnsafeCell<T>,
    v2: UnsafeCell<T>,

    flag_: atomic::AtomicBool,
}

impl<T> ReceiveCell<T>
where
    T: Copy,
{
    fn new(initial: T) -> ReceiveCell<T> {
        ReceiveCell {
            v1: UnsafeCell::new(initial),
            v2: UnsafeCell::new(initial),
            flag_: atomic::AtomicBool::new(false),
        }
    }

    fn set(&self, val: T) {
        if self.flag_.load(atomic::Ordering::Acquire) {
            unsafe {
                *self.v1.get() = val;
            }
            self.flag_.store(false, atomic::Ordering::Release);
        } else {
            unsafe {
                *self.v2.get() = val;
            }
            self.flag_.store(true, atomic::Ordering::Release);
        }
    }

    fn read(&self) -> T {
        if self.flag_.load(atomic::Ordering::Relaxed) {
            unsafe { *self.v2.get() }
        } else {
            unsafe { *self.v1.get() }
        }
    }
}

/// this is a lie. you can only have ONE writer
unsafe impl<T> Sync for ReceiveCell<T>
where
    T: Copy,
{
}


/// sender.
pub struct Sender<T> {
    receiver: Weak<ReceiveCell<T>>,
}

#[derive(Debug)]
pub enum SendStatus {
    Disconnected,
}

impl<T> Sender<T>
where
    T: Copy,
{
    pub fn send(&self, val: T) -> Result<(), SendStatus> {
        if let Some(rcv) = self.receiver.upgrade() {
            rcv.set(val);
            Result::Ok(())
        } else {
            Result::Err(SendStatus::Disconnected)
        }
    }
}



pub struct Reader<T> {
    v: Arc<ReceiveCell<T>>,
}

impl<T> Reader<T>
where
    T: Copy,
{
    pub fn value(&self) -> T {
        self.v.read()
    }
}

/// create disconnectable writer/receiver pair.
pub fn link<T: Copy>(default: T) -> (Sender<T>, Reader<T>) {
    let intern_ = Arc::new(ReceiveCell::new(default));
    (
        Sender { receiver: Arc::downgrade(&intern_) },
        Reader { v: intern_ },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::spawn;

    #[test]
    fn test_reader() {

        let (writer, reader) = link::<f64>(42.0);

        assert_eq!(42.0, reader.value());

        let hdle = spawn(move || {
            writer.send(5.0).unwrap();
            writer.send(6.0).unwrap();
        });
        // watch for the fucking race condition
        assert_eq!(42.0, reader.value());
        hdle.join().unwrap();
        assert_eq!(6.0, reader.value());

    }
}
