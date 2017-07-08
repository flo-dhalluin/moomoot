//! Sine wave generator with frequency configuration exposed through standard input.
extern crate moomoot;

use std::io;
use std::str::FromStr;
use std::sync::mpsc::channel;


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

    let (cmd_chan, moomoot) = moomoot::MooMoot::start();

    // 5. wait or do some processing while your handler is running in real time.
    println!("Enter an integer value to change the frequency of the sine wave.");
    while let Some(f) = read_freq() {
        cmd_chan.send( moomoot::MooMootCmd::AddSynth(String::from("kps") )).unwrap();
    }

}
