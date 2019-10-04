use std::process;
use std::env::args;
use std::borrow::Cow;
use std::io::{ self, Write };
use ttyaskpass::AskPass;


#[inline]
fn start(prompt: &str) -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut askpass = AskPass::new(vec![0; 256].into_boxed_slice());
    let pass = askpass.askpass(prompt)?;
    stdout.write_all(pass)?;
    stdout.flush()
}

fn main() {
    let prompt = args().nth(1)
        .map(Cow::from)
        .unwrap_or(Cow::from("Password:"));
    match start(&prompt) {
        Ok(()) => (),
        Err(ref err) if err.kind() == io::ErrorKind::Interrupted => process::exit(1),
        err => err.unwrap()
    }
}
