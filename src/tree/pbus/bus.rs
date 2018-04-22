use super::sender::{Sender, Reader, link};

pub struct Bus<T> {
    senders: Vec<Sender<T>>,
    initial_value: T,
}

impl<T> Bus<T>
where
    T: Copy,
{
    pub fn new(initial_value: T) -> Bus<T> {
        Bus {
            senders: Vec::new(),
            initial_value: initial_value,
        }
    }

    pub fn subscribe(&mut self) -> Reader<T> {
        let (send, reader) = link::<T>(self.initial_value);
        self.senders.push(send);
        reader
    }

    pub fn publish(&mut self, value: T) {
        self.initial_value = value;
        self.senders.retain(|sender| sender.send(value).is_ok())
    }

    pub fn sub_count(&self) -> usize {
        self.senders.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bus() {
        let mut bus = Bus::new(42.0);

        let r1 = bus.subscribe();

        assert_eq!(42.0, r1.value());
        bus.publish(1.);
        assert_eq!(1.0, r1.value());
        assert_eq!(1, bus.sub_count());
        {
            let r2 = bus.subscribe();
            assert_eq!(1.0, r2.value());
            assert_eq!(2, bus.sub_count());
        }

        bus.publish(2.); // need to propagate a value to update subcount
        assert_eq!(1, bus.sub_count());
    }
}
