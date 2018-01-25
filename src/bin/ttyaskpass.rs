extern crate seckey;
extern crate ttyaskpass;

use std::process;
use std::env::args;
use std::borrow::Cow;
use std::io::{ self, Write };
use ttyaskpass::askpass;


#[inline]
fn start(prompt: &str) -> io::Result<()> {
    let mut stdout = io::stdout();
    askpass(prompt, |pass| write!(stdout, "{}", pass))?;
    stdout.flush()
}

fn main() {
    let prompt = args().nth(1)
        .map(Cow::from)
        .unwrap_or(Cow::from("Password:"));
    match start(&prompt) {
        Ok(()) => (),
        Err(ref err) if err.kind() == io::ErrorKind::Interrupted => process::exit(1),
        err => err.unwrap()
    }
}
