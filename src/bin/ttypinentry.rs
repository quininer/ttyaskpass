extern crate url;
extern crate nom;
extern crate libc;
#[macro_use] extern crate ttyaskpass;

use std::{ str, process };
use std::io::{ self, Read, Write, BufRead, BufReader };
use url::percent_encoding::percent_decode;
use nom::IError;
use ttyaskpass::pinentry::{ Pinentry, Button, parse_command, parse_option };
use ttyaskpass::utils::code::*;


#[inline]
fn start(input: &mut Read, output: &mut Write) -> io::Result<()> {
    let mut input = BufReader::new(input);
    let mut pinentry = Pinentry::default();
    let mut buf = String::new();

    dump!(OK: output, START)?;

    loop {
        buf.clear();
        input.read_line(&mut buf)?;

        let (command, value) = match parse_command(buf.as_bytes()).to_full_result() {
            Ok((cmd, value)) => (
                cmd.to_uppercase(),
                percent_decode(value.as_bytes())
                    .decode_utf8()
                    .map_err(|err| err!(Other, err))?
            ),
            Err(err) => match err {
                IError::Error(err) => {
                    dump!(ERR: output, err)?;
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
                dump!(ERR: output, USER_NOT_IMPLEMENTED)?;
                continue
            },
            "OPTION" => match parse_option(value.as_bytes()).to_full_result() {
                Ok(("ttyname", value)) => pinentry.tty = value.into(),
                Ok(_) => { /* ignore */ },
                _ => {
                    dump!(ERR: output, PINENTRY_UNKNOWN_OPTION)?;
                    continue
                }
            },

            "GETPIN" => {
                let pin = pinentry.get_pin()?;
                if !pinentry.repeat.is_empty() {
                    dump!(S: output, "PIN_REPEATED")?;
                }
                dump!(D: output, unsafe { str::from_utf8_unchecked(&pin) })?;
                    //                      ^- SAFE: because pin from `String`.
            },
            cmd @ "CONFIRM" | cmd @ "MESSAGE" => {
                match pinentry.confirm(cmd == "MESSAGE" || value == "--one-button")? {
                    Button::Ok => dump!(OK: output),
                    Button::Cancel => dump!(ERR: output, PINENTRY_OPERATION_CANCELLED),
                    Button::NotOk => dump!(ERR: output, PINENTRY_NOT_CONFIRMED)
                }?;
                continue
            },
            "GETINFO" => match value.as_ref() {
                "version" => dump!(D: output, env!("CARGO_PKG_VERSION")),
                "pid" => dump!(D: output, unsafe { ::libc::getpid() }),
                "flavor" => dump!(D: output, "tty"),
                "ttyinfo" => dump!(D: output, "- - -"),
                _ => dump!(ERR: output, PINENTRY_PARAMETER_ERROR)
            }?,
            "BYE" => {
                dump!(OK: output, CLOSE)?;
                return Ok(())
            },

            "" => continue,
            _ => {
                dump!(ERR: output, USER_UNKNOWN_COMMAND)?;
                continue
            }
        }

        dump!(OK: output)?;
    }
}

fn main() {
    match start(&mut io::stdin(), &mut io::stdout()) {
        Ok(()) => (),
        Err(ref err) if err.kind() == io::ErrorKind::Interrupted => process::exit(1),
        Err(ref err) if err.kind() == io::ErrorKind::ConnectionAborted => process::exit(1),
        err => err.unwrap()
    }
}
