extern crate url;
extern crate nom;
#[macro_use] extern crate ttyaskpass;

use std::{ str, process };
use std::io::{ self, Write };
use url::percent_encoding::percent_decode;
use nom::IError;
use ttyaskpass::pinentry::{ Pinentry, Button, parse_command, parse_option };
use ttyaskpass::utils::*;


#[inline]
fn start() -> io::Result<()> {
    let mut pinentry = Pinentry::default();
    let mut buf = String::new();

    dump!(OK: io::stdout(), START)?;

    loop {
        buf.clear();
        io::stdin().read_line(&mut buf)?;

        let (command, value) = match parse_command(buf.as_bytes()).to_full_result() {
            Ok((cmd, value)) => (
                cmd.to_uppercase(),
                percent_decode(value.as_bytes())
                    .decode_utf8()
                    .map_err(|err| err!(Other, err))?
            ),
            Err(err) => match err {
                IError::Error(err) => {
                    dump!(ERR: io::stdout(), err)?;
                    continue
                },
                IError::Incomplete(_) => return Err(err!(ConnectionAborted))
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
            "SETTIMEOUT" | "CLEARPASSPHRASE" => {
                dump!(ERR: io::stdout(), USER_NOT_IMPLEMENTED)?;
                continue
            },
            "OPTION" => match parse_option(value.as_bytes()).to_full_result() {
                Ok(("ttyname", value)) => pinentry.tty = value.into(),
                Ok(_) => { /* ignore */ },
                _ => {
                    dump!(ERR: io::stdout(), PINENTRY_UNKNOWN_OPTION)?;
                    continue
                }
            },

            "GETPIN" => {
                let pin = pinentry.get_pin()?;
                if !pinentry.repeat.is_empty() {
                    dump!(S: io::stdout(), "PIN_REPEATED")?;
                }
                dump!(D: io::stdout(), unsafe { str::from_utf8_unchecked(&pin) })?;
                    //                      ^- SAFE: because pin from `String`.
            },
            cmd @ "CONFIRM" | cmd @ "MESSAGE" => {
                match pinentry.confirm(cmd == "MESSAGE" || value == "--one-button")? {
                    Button::Ok => dump!(OK: io::stdout()),
                    Button::Cancel => dump!(ERR: io::stdout(), PINENTRY_OPERATION_CANCELLED),
                    Button::NotOk => dump!(ERR: io::stdout(), PINENTRY_NOT_CONFIRMED)
                }?;
                continue
            },
            "GETINFO" => pinentry.get_info(&mut io::stdout(), &value)?,
            "BYE" => {
                dump!(OK: io::stdout(), CLOSE)?;
                return Ok(())
            },

            "" => continue,
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
        Err(ref err) if err.kind() == io::ErrorKind::ConnectionAborted => process::exit(1),
        err => err.unwrap()
    }
}
