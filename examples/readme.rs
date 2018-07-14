extern crate ttyaskpass;

use std::io;
use ttyaskpass::askpass;


fn main() {
    askpass("Password:", |pass| -> io::Result<()> {
        print!("Your password is {}", pass);

        Ok(())
    }).unwrap();
}
