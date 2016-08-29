extern crate ttyaskpass;

use std::io::{ self, Write };
use ttyaskpass::askpass;

fn main() {
    let mut stdout = io::stdout();
    stdout.write(&askpass('~')).unwrap();
    stdout.flush().unwrap();
}
