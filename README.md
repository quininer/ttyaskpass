# ttyaskpass
[![travis-ci](https://travis-ci.org/quininer/ttyaskpass.svg?branch=master)](https://travis-ci.org/quininer/ttyaskpass)
[![crates](https://img.shields.io/crates/v/ttyaskpass.svg)](https://crates.io/crates/ttyaskpass)
[![license](https://img.shields.io/github/license/quininer/ttyaskpass.svg)](https://github.com/quininer/ttyaskpass/blob/master/LICENSE)
[![docs.rs](https://docs.rs/ttyaskpass/badge.svg)](https://docs.rs/ttyaskpass/)

![ttyaskpass](ttyaskpass.png)

A safely passphrase prompt library and application,
support [Chroma-Hash](https://github.com/mattt/Chroma-Hash/)-like colorhash,
use [seckey](https://github.com/quininer/seckey) protecte password.

usage
-----

library:

```rust
extern crate ttyaskpass;

use std::io;
use ttyaskpass::askpass;

fn main() {
    askpass("Password:", |pass| -> io::Result<()> {
        print!("Your password is {}", pass);

        Ok(())
    }).unwrap();
}
```

see [readme.rs](examples/readme.rs) and [ttyaskpass.rs](src/bin/ttyaskpass.rs).


application:

```bash
env SSH_ASKPASS=ttyaskpass ssh-add </dev/null
```
