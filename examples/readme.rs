extern crate ttyaskpass;
use ttyaskpass::askpass;

fn main() {
    askpass("Password:", |pass| {
        print!("Your password is {}", pass);

        Ok(())
    }).unwrap();
}
