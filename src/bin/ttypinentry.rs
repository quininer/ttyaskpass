#[macro_use] extern crate ttyaskpass;

use std::process;
use std::io::{ self, Write };
use ttyaskpass::pinentry::{ Pinentry, Command };


#[inline]
fn start() -> io::Result<()> {
    let mut pinentry = Pinentry::default();
    let mut buf = String::new();

    writeln!(io::stdout(), "OK Pleased to meet you")?;

    loop {
        buf.clear();
        io::stdin().read_line(&mut buf)?;

        match Command::parse(buf.as_bytes()) {
            Err(err) => {
                dump!(ERR: io::stdout(), err)?;
                continue
            },

            Ok(Command::SetDesc(desc)) => pinentry.description = desc,
            Ok(Command::SetPrompt(prompt)) => pinentry.prompt = prompt,
            Ok(Command::SetKeyInfo(keyinfo)) => pinentry.keyinfo = keyinfo,
            Ok(Command::SetRepeat(repeat)) => pinentry.repeat = repeat,
            Ok(Command::SetRepeatError(error)) => pinentry.repeat_error = error,
            Ok(Command::SetError(error)) => pinentry.error = error,
            Ok(Command::SetOk(ok)) => pinentry.ok = ok,
            Ok(Command::SetNotOk(notok)) => pinentry.not_ok = notok,
            Ok(Command::SetCancel(cancel)) => pinentry.cancel = cancel,
            Ok(Command::SetQualityBar(q)) => pinentry.quality_bar = q,
            Ok(Command::SetQualityBarTT(q)) => pinentry.quality_bar_tt = q,
            Ok(Command::SetTitle(title)) => pinentry.title = title,
            Ok(Command::SetTimeout(_)) => {
                dump!(ERR: io::stdout(), "unimplemented")?;
                continue
            },

            Ok(Command::GetPin) => pinentry.get_pin(&mut io::stdout())?,
            Ok(Command::Confirm) => (),
            Ok(Command::Message) => (),
            Ok(Command::GetInfo(_)) => (),
            Ok(Command::ClearPassphrase) => ()
        }

        dump!(OK: io::stdout())?;
    }
}

fn main() {
    match start() {
        Ok(()) => (),
        Err(ref err) if err.kind() == io::ErrorKind::Interrupted => process::exit(1),
        err => err.unwrap()
    }
}
