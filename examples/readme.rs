extern crate ttyaskpass;
use ttyaskpass::askpass;

fn main() {
	let pass = askpass::<Vec<u8>>("Password:", '*').unwrap();
	println!("Your password is {}", String::from_utf8(pass).unwrap());
}
