# ttyaskpass
[![travis-ci](https://travis-ci.org/quininer/ttyaskpass.svg?branch=master)](https://travis-ci.org/quininer/ttyaskpass)
[![crates](https://img.shields.io/crates/v/ttyaskpass.svg)](https://crates.io/crates/ttyaskpass)
[![license](https://img.shields.io/github/license/quininer/ttyaskpass.svg)](https://github.com/quininer/ttyaskpass/blob/master/LICENSE)
[![docs.rs](https://docs.rs/ttyaskpass/badge.svg)](https://docs.rs/ttyaskpass/)

![ttyaskpass](ttyaskpass.png)

A safely passphrase prompt library and application,
support [Chroma-Hash](https://github.com/mattt/Chroma-Hash/)-like colorhash.

usage
-----

library:

```rust
use std::io::{ self, Write };
use ttyaskpass::AskPass;


fn main() -> io::Result<()> {
    let mut cli = AskPass::new([0; 32]);
    let pass = cli.askpass("Password:")?;

    let mut stdout = io::stdout();
    write!(&mut stdout, "Your password is ")?;
    stdout.write_all(pass)?;
    stdout.flush()?;

    Ok(())
}
```

see [readme.rs](examples/readme.rs) and [ttyaskpass.rs](src/bin/ttyaskpass.rs).


application:

```bash
env SSH_ASKPASS=ttyaskpass ssh-add </dev/null
```
