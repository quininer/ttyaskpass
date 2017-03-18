extern crate seckey;
extern crate ttyaskpass;

use std::process;
use std::env::args;
use std::io::{ self, Write };
use seckey::Bytes;
use ttyaskpass::askpass;


#[inline]
fn start() -> io::Result<()> {
    let prompt = args().skip(1).fold(String::new(), |sum, next| sum + &next);

    match askpass::<Bytes>(if !prompt.is_empty() { &prompt } else { "Password:" }, '~') {
        Ok(output) => {
            let mut stdout = io::stdout();
            stdout.write(&output)?;
            stdout.flush()?;
            Ok(())
        },
        Err(ref err) if err.kind() == io::ErrorKind::Interrupted => process::exit(1),
        Err(err) => Err(err)
    }
}

fn main() {
    start().unwrap();
}
