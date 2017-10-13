use tree::pbus;
use std::fmt;
use std::cmp;

impl<T> fmt::Debug for pbus::Reader<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bus::Receiver_xxx")
    }
}

// dummy partial Eq implemenation so we can have tests
impl<T> cmp::PartialEq for pbus::Reader<T> {
    fn eq(&self, other: &pbus::Reader<T>) -> bool {
        false
    }
}

#[derive(Debug, PartialEq)]
pub enum BusParam {
    NotConnected(String),
    Connected(pbus::Reader<f64>)
}

/// Parameters from the client side.
#[derive(Debug, PartialEq)]
pub enum ParamValue {
    /// a constant value
    Constant(f64),
    /// an internal bus id (for adjustable parameters)
    BusValue(BusParam),
    /// using the Synth's default value.
    Default(f64),
}

impl ParamValue {
    pub fn default(v: f64) -> ParamValue {
        ParamValue::Default(v)
    }

    fn connect(&mut self, buses: &mut pbus::BusSystem) {
        let recvr = { if let ParamValue::BusValue(BusParam::NotConnected(ref busid)) = *self {
            Some(buses.sub(&busid))
        } else {
            None
        }
    };

        if recvr.is_some() {
            *self = ParamValue::BusValue(BusParam::Connected(recvr.unwrap()));
        }
    }

    pub fn value(&self) -> f64 {
        match *self {
            ParamValue::Constant(ref x) => *x,
            ParamValue::Default(ref x) => *x,
            ParamValue::BusValue(ref x) => {
                if let BusParam::Connected(ref rcvr) = *x {
                    rcvr.value()
                } else {
                    panic!("un-connected bus");
                }
            }
        }
    }
}

impl From<f64> for ParamValue {
    fn from(val: f64) -> ParamValue {
        ParamValue::Constant(val)
    }
}

impl<'a> From<&'a str> for ParamValue {
    fn from(bus: &'a str) -> ParamValue {
        ParamValue::BusValue(BusParam::NotConnected(bus.to_string()))
    }
}

pub trait Parameters {
    //fn list_parameters_names(&self) -> Vec<&str>;
    fn map_parameters(&mut self) -> Vec<&mut ParamValue>;
}

struct NoParameters;

impl Parameters for NoParameters {
    fn map_parameters(&mut self) -> Vec<&mut ParamValue> {
        Vec::new()
    }
}

macro_rules! declare_params {
    ($name:ident {$($p:ident : $v:expr),* }) => {

    #[derive(Debug)]
    pub struct $name {
        $(
            $p : ParamValue,
        )*
    }

    impl Default for $name {
        fn default() -> $name {
            $name {
                $(
                $p: ParamValue::default($v),
                )*
            }
        }
    }

    impl Parameters for $name {
        /*
        fn list_parameters_names(&self) -> Vec<&str> {
            vec![ $( $p ,)*]
        }
        */
        fn map_parameters(&mut self) -> Vec<&mut ParamValue> {
            vec![$(&mut self.$p,)*]
        }
    }

    impl $name {
        $(
            #[allow(dead_code)]
            pub fn $p<T>(mut self, v: T) -> $name
                where ParamValue: From<T> {
                    self.$p = ParamValue::from(v);
                    self
            }
        )*
    }
}}

static mut NO_PARAMETERS: NoParameters = NoParameters{};

pub trait Parametrized {

    fn get_parameters(&mut self) -> &mut Parameters {
        unsafe { &mut NO_PARAMETERS }
    }

    /// connects bus parameters to
    fn connect_parameters(&mut self, buses: &mut pbus::BusSystem) {
        for p in self.get_parameters().map_parameters() {
            p.connect(buses);
        }
    }
}

#[cfg(test)]
mod tests {
    #[macro_use]
    use super::*;

    declare_params!(SomeParams {a : 42.0, b : 77.0});

    #[derive(Default)]
    struct Chombier(SomeParams);

    impl Parametrized for Chombier {
        fn get_parameters(&mut self) -> &mut Parameters {
            return &mut self.0;
        }
    }

    impl Chombier {
        fn tic(&self) -> f64 {
            self.0.a.value() + self.0.b.value()
        }
    }

    #[test]
    fn test_default_values() {

        let p = SomeParams::default().b(5.0);

        assert_eq!(p.a, ParamValue::Default(42.0));
        assert_eq!(p.b, ParamValue::Constant(5.0));

    }

    #[test]
    fn test_connection() {
        let c = Chombier::default();
        assert_eq!(c.tic(), 119.0);

        let mut bus = pbus::BusSystem::new();
        let mut cc = Chombier(SomeParams::default().a(2.0).b("bite"));
        cc.connect_parameters(&mut bus);
        bus.publish("bite", 2.0);
        assert_eq!(cc.tic(), 4.0);
        bus.publish("bite", 4.0);
        assert_eq!(cc.tic(), 6.0);


    }


}
