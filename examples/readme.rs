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
