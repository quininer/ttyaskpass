extern crate seckey;
extern crate ttyaskpass;

use std::process;
use std::io::{ self, Write };
use seckey::Bytes;
use ttyaskpass::askpass;


fn start() -> io::Result<()> {
    match askpass::<Bytes>('~') {
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
