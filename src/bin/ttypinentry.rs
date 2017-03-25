#[macro_use] extern crate ttyaskpass;

use std::process;
use std::io::{ self, Write };
use ttyaskpass::pinentry::{ Pinentry, Button, parse_command };
use ttyaskpass::utils::*;


#[inline]
fn start() -> io::Result<()> {
    let mut pinentry = Pinentry::default();
    let mut buf = String::new();

    dump!(OK: io::stdout(), START)?;

    loop {
        buf.clear();
        io::stdin().read_line(&mut buf)?;

        let (command, value) = match parse_command(buf.as_bytes()).to_result() {
            Ok((cmd, value)) => (cmd.to_uppercase(), value),
            Err(err) => {
                dump!(ERR: io::stdout(), err)?;
                continue
            }
        };

        match command.as_ref() {
            "SETDESC" => pinentry.description = value.into(),
            "SETPROMPT" => pinentry.prompt = value.into(),
            "SETKEYINFO" => pinentry.keyinfo = value.into(),
            "SETREPEAT" => pinentry.repeat = value.into(),
            "SETREPEATERROR" => pinentry.repeat_error = value.into(),
            "SETERROR" => pinentry.error = value.into(),
            "SETOK" => pinentry.ok = value.into(),
            "SETNOTOK" => pinentry.notok = value.into(),
            "SETCANCEL" => pinentry.cancel = value.into(),
            "SETQUALITYBAR" => pinentry.quality_bar = value.into(),
            "SETQUALITYBAR_TT" => pinentry.quality_bar_tt = value.into(),
            "SETTITLE" => pinentry.title = value.into(),
            "SETTIMEOUT" => {
                dump!(ERR: io::stdout(), USER_NOT_IMPLEMENTED)?;
                continue
            },

            "GETPIN" => pinentry.get_pin(&mut io::stdout())?,
            cmd @ "CONFIRM" | cmd @ "MESSAGE" => {
                match pinentry.confirm(cmd == "MESSAGE")? {
                    Button::Ok => dump!(OK: io::stdout()),
                    Button::Cancel => dump!(ERR: io::stdout(), PINENTRY_OPERATION_CANCELLED),
                    Button::NotOk => dump!(ERR: io::stdout(), PINENTRY_NOT_CONFIRMED)
                }?;
                continue
            },
            "GETINFO" => (),
            "CLEARPASSPHRASE" => (),

            _ => {
                dump!(ERR: io::stdout(), USER_UNKNOWN_COMMAND)?;
                continue
            }
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
