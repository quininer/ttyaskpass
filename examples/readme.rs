extern crate ttyaskpass;
use ttyaskpass::askpass;

fn main() {
	askpass("Password:", |pass| {
        print!("Your password is {}", pass.iter().collect::<String>());

        Ok(())
    }).unwrap();
}
