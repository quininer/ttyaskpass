# ttyaskpass
[![travis-ci](https://travis-ci.org/quininer/ttyaskpass.svg?branch=master)](https://travis-ci.org/quininer/ttyaskpass)
[![crates](https://img.shields.io/crates/v/ttyaskpass.svg)](https://crates.io/crates/ttyaskpass)
[![license](https://img.shields.io/github/license/quininer/ttyaskpass.svg)](https://github.com/quininer/ttyaskpass/blob/master/LICENSE)
[![docs.rs](https://docs.rs/ttyaskpass/badge.svg)](https://docs.rs/ttyaskpass/)

![ttyaskpass](ttyaskpass.png)

a safely passphrase prompt library and application,
support [Chroma-Hash](https://github.com/mattt/Chroma-Hash/)-like colorhash,
use [seckey](https://github.com/quininer/seckey) protecte password.

usage
-----

library:

```rust
extern crate ttyaskpass;
use ttyaskpass::askpass;

fn main() {
	let pass = askpass::<Vec<u8>>("Password:", '*').unwrap();
	println!("Your password is {}", String::from_utf8(pass).unwrap());
}
```

see [examples/readme.rs](examples/readme.rs) and [ttyaskpass.rs](src/bin/ttyaskpass.rs).


application:

```bash
env SSH_ASKPASS=ttyaskpass ssh-add </dev/null
```
