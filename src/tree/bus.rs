use std::rc::Weak;
use std::rc::Rc;
use std::cell::Cell;
use std::error;
use std::fmt;
use std::collections::HashMap;

struct ReceiverBox<T> {
    last_value: Cell<T>,
}

pub struct Receiver<T> {
    // ok now can we avoid the Rc / Cell crap with nice RAII ?
    inner: Rc<ReceiverBox<T>>,
}

impl<T> Receiver<T>
where
    T: Copy,
{
    pub fn value(&self) -> T {
        self.inner.last_value.get()
    }
}

/// A bus dispatch every call to "publish" to all it's receivers
/// example :
/// let mut bus = Bus::new("bus", 2);
/// let recv = bus.sub();
/// assert!(recv.value() == 2);
/// bus.publish(15);
/// assert!(recv.value() == 15);
pub struct Bus<T> {
    id: String,
    last_value: T,
    receivers: Vec<Weak<ReceiverBox<T>>>,
}

impl<T> Bus<T>
where
    T: Copy,
{
    pub fn new(id: &str, initial_value: T) -> Bus<T> {
        Bus {
            id: id.to_string(),
            receivers: Vec::new(),
            last_value: initial_value,
        }
    }

    pub fn publish(&mut self, value: T) {
        // update receivers, or remove them if dead.
        self.receivers.retain(
            |w_recv| if let Some(rcv) = w_recv.upgrade() {
                rcv.last_value.set(value);
                true
            } else {
                false
            },
        );
        self.last_value = value;
    }

    pub fn sub(&mut self) -> Receiver<T> {
        let receiver = Rc::new(ReceiverBox { last_value: Cell::new(self.last_value) });
        self.receivers.push(Rc::downgrade(&receiver));
        Receiver { inner: receiver }
    }

    pub fn subscriber_count(&self) -> usize {
        self.receivers.len()
    }
}

#[derive(Debug)]
pub enum BusError {
    NoSuchChannel(String),
}

impl fmt::Display for BusError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BusError::NoSuchChannel(ref chan) => {
                fmt.debug_struct("NoSuchChannel")
                    .field("channel", &chan)
                    .finish()
            }
        }
    }
}

impl error::Error for BusError {
    fn description(&self) -> &str {
        match *self {
            BusError::NoSuchChannel(_) => "No such channel",
        }
    }
}

pub struct BusSystem {
    busses: HashMap<String, Bus<f64>>,
}


impl BusSystem {
    // ok, there's a big fat leak : when there are no listenner to a bus, it stays in the map.
    pub fn new() -> BusSystem {
        BusSystem { busses: HashMap::new() }
    }

    // ideally sub<T> -> Receiver<T>
    pub fn sub(&mut self, chan: &str) -> Receiver<f64> {

        self.busses
            .entry(chan.to_string())
            .or_insert(Bus::new(chan, 0.0))
            .sub()
    }

    pub fn publish(&mut self, chan: &str, value: f64) -> Result<(), BusError> {
        self.busses.get_mut(chan).map(|c| c.publish(value)).ok_or(
            BusError::NoSuchChannel(chan.to_string()),
        )
    }
}
