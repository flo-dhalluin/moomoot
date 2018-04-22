mod sender;
mod bus;

use std::error;
use std::fmt;
use std::collections::HashMap;

pub use self::sender::Reader;

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
    busses: HashMap<String, bus::Bus<f64>>,
}

impl BusSystem {
    // ok, there's a big fat leak : when there are no listenner to a bus, it stays in the map.
    pub fn new() -> BusSystem {
        BusSystem { busses: HashMap::new() }
    }

    // ideally sub<T> -> Receiver<T>
    pub fn sub(&mut self, chan: &str) -> Reader<f64> {

        self.busses
            .entry(chan.to_string())
            .or_insert(bus::Bus::new(0.0))
            .subscribe()
    }

    pub fn publish(&mut self, chan: &str, value: f64) -> Result<(), BusError> {
        self.busses.get_mut(chan).map(|c| c.publish(value)).ok_or(
            BusError::NoSuchChannel(chan.to_string()),
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    struct Stuff {
        a: Reader<f64>,
        b: Reader<f64>,
    }


    impl Stuff {
        fn doit(&self) -> f64 {
            self.a.value() + self.b.value()
        }
    }

    #[test]
    fn test_bus_system() {
        let mut bus = BusSystem::new();

        let stuff = Stuff {
            a: bus.sub("a"),
            b: bus.sub("b"),
        };

        bus.publish("a", 2.0).unwrap();
        bus.publish("b", 4.0).unwrap();

        assert_eq!(stuff.doit(), 6.0);

        assert!(bus.publish("b", 5.0).is_ok());
        assert!(bus.publish("d", 5.0).is_err());
    }
}
