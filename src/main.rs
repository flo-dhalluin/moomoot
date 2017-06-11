//! Sine wave generator with frequency configuration exposed through standard input.
extern crate jack;
extern crate moomoot;

use std::io;
use std::str::FromStr;
use std::sync::mpsc::channel;
use jack::prelude::{Client, AsyncClient, client_options};
use moomoot::synth::noise::WhiteNoise;
use moomoot::synth::string::KarplusStrong;
use moomoot::synth::sine::Sine;


/// Attempt to read a frequency from standard in. Will block until there is user input. `None` is
/// returned if there was an error reading from standard in, or the retrieved string wasn't a
/// compatible u16 integer.
fn read_freq() -> Option<f64> {
    let mut user_input = String::new();
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => u16::from_str(&user_input.trim()).ok().map(|n| n as f64),
        Err(_) => None,
    }
}




fn main() {
    // 1. open a client
    let (client, _status) = Client::new("rust_jack_sine", client_options::NO_START_SERVER).unwrap();

    let sample_rate = client.sample_rate() as f64;
    let (tx, rx) = channel();


    let mut process = moomoot::MooMoot::new(&client);

    // processing starts here
    process.add_synth(Box::new(Sine::new(sample_rate, 440.)));
    process.add_synth(Box::new(Sine::new(sample_rate, 220.)));
    process.add_synth(Box::new(Sine::new(sample_rate, 50.)));

    // 4. activate the client
    let active_client = AsyncClient::new(client, (), process).unwrap();


    // 5. wait or do some processing while your handler is running in real time.
    println!("Enter an integer value to change the frequency of the sine wave.");
    while let Some(f) = read_freq() {
        tx.send(f).unwrap();
    }

    // 6. Optional deactivate. Not required since active_client will deactivate on drop, though
    // explicit deactivate may help you identify errors in deactivate.
    active_client.deactivate().unwrap();
}
