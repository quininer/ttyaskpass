extern crate seckey;
extern crate ttyaskpass;

use std::io::{ self, Write };
use seckey::Bytes;
use ttyaskpass::askpass;

fn main() {
    let mut stdout = io::stdout();
    stdout.write(&askpass::<Bytes>('~')).unwrap();
    stdout.flush().unwrap();
}
