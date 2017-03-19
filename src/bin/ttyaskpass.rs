extern crate seckey;
extern crate ttyaskpass;

use std::process;
use std::env::args;
use std::io::{ self, Write };
use seckey::Bytes;
use ttyaskpass::askpass;


#[inline]
fn start(prompt: &str) -> io::Result<()> {
    match askpass::<Bytes>(prompt, '~') {
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
    let prompt = args().skip(1).next().unwrap_or("Password:".into());
    start(&prompt).unwrap()
}
