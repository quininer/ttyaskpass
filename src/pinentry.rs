#![cfg(feature = "pinentry")]

use std::str;
use std::borrow::Cow;
use std::time::Duration;
use std::io::{ self, Write };
use nom::{ space, is_alphabetic };
use termion::get_tty;
use termion::color::{ Fg, Red, Reset };
use seckey::Bytes;
use super::readtty::RawTTY;
use super::raw_askpass;


#[macro_export]
macro_rules! dump {
    ( WARN : $tty:expr, $message:expr ) => {
        writeln!(
            $tty,
            "\n\r *** {}{}{} ***\n",
            Fg(Red), $message, Fg(Reset)
        )
    };
    ( S : $tty:expr, $message:expr ) => {
        writeln!($tty, "S {}", $message)
    };
    ( D : $tty:expr, $message:expr ) => {
        writeln!($tty, "D {}", $message)
    };
    ( OK : $tty:expr ) => {
        writeln!($tty, "OK")
    };
    ( ERR : $tty:expr, $message:expr ) => {
        writeln!($tty, "ERR {}", $message)
    };
    ( PRINT: $tty:expr, $message:expr ) => {
        writeln!($tty, "\r{}", $message)
    }
}


#[derive(Debug, Clone, Default)]
pub struct Pinentry {
    pub description: String,
    pub prompt: String,
    pub keyinfo: String,
    pub repeat: String,
    pub repeat_error: String,
    pub error: String,
    pub ok: String,
    pub not_ok: String,
    pub cancel: String,
    pub quality_bar: String,
    pub quality_bar_tt: String,
    pub title: String,
    pub timeout: Option<Duration>
}

impl Pinentry {
    pub fn get_pin(&self, output: &mut Write) -> io::Result<()> {
        let (mut raw_tty, mut tty) = (RawTTY::new()?, get_tty()?);
        let mut pin;

        let message =
            if !self.description.is_empty() { &self.description }
            else if !self.title.is_empty() { &self.title }
            else { "Enter your passphrase" };
        let prompt =
            if !self.prompt.is_empty() { Cow::from(format!("{}:", self.prompt.trim_right_matches(':'))) }
            else { Cow::from("Password:") };

        if !self.error.is_empty() {
            dump!(WARN: tty, self.error)?;
        }

        dump!(PRINT: tty, message)?;

        loop {
            pin = raw_askpass(&mut raw_tty, &mut tty, &prompt, '*')
                .map(|p| Bytes::from(p.into_bytes()))?;

            if !self.repeat.is_empty() {
                let repeat_error =
                    if !self.repeat_error.is_empty() { &self.repeat_error }
                    else { "Passphrases don't match." };
                let pin2 = raw_askpass(&mut raw_tty, &mut tty, &self.repeat, '*')
                    .map(|p| Bytes::from(p.into_bytes()))?;

                if pin != pin2 {
                    dump!(WARN: tty, repeat_error)?;
                    continue
                }
            }

            break
        }

        drop((raw_tty, tty));

        if !self.repeat.is_empty() {
            dump!(S: output, "PIN_REPEATED")?;
        }
        dump!(D: output, unsafe { str::from_utf8_unchecked(&pin) })?;
            // ^- SAFE: because pin from `String`.

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    SetDesc(String),
    SetPrompt(String),
    SetKeyInfo(String),
    SetRepeat(String),
    SetRepeatError(String),
    SetError(String),
    SetOk(String),
    SetNotOk(String),
    SetCancel(String),
    GetPin,
    Confirm,
    Message,
    SetQualityBar(String),
    SetQualityBarTT(String),
    GetInfo(String),
    SetTitle(String),
    SetTimeout(Duration),
    ClearPassphrase
}

impl Command {
    #[inline]
    pub fn parse(input: &[u8]) -> io::Result<Self> {
        let (command, value) = parse_command(input)
            .to_result()
            .map_err(|err| err!(Other, err))?;
        Self::from(command, value)
    }

    pub fn from(command: &str, value: &str) -> io::Result<Self> {
        match command {
            "SETDESC" => Ok(Command::SetDesc(value.into())),
            "SETPROMPT" => Ok(Command::SetPrompt(value.into())),
            "SETKEYINFO" => Ok(Command::SetKeyInfo(value.into())),
            "SETREPEAT" => Ok(Command::SetRepeat(value.into())),
            "SETREPEATERROR" => Ok(Command::SetRepeatError(value.into())),
            "SETERROR" => Ok(Command::SetError(value.into())),
            "SETOK" => Ok(Command::SetOk(value.into())),
            "SETNOTOK" => Ok(Command::SetNotOk(value.into())),
            "SETCANCEL" => Ok(Command::SetCancel(value.into())),
            "GETPIN" => Ok(Command::GetPin),
            "CONFIRM" => Ok(Command::Confirm),
            "MESSAGE" => Ok(Command::Message),
            "SETQUALITYBAR" => Ok(Command::SetQualityBar(value.into())),
            "SETQUALITYBAR_TT" => Ok(Command::SetQualityBarTT(value.into())),
            "GETINFO" => Ok(Command::GetInfo(value.into())),
            "SETTITLE" => Ok(Command::SetTitle(value.into())),
            "SETTIMEOUT" => Ok(Command::SetTimeout(Duration::from_secs(value.parse().map_err(|err| err!(Other, err))?))),
            "CLEARPASSPHRASE" => Ok(Command::ClearPassphrase),
            _ => Err(err!(Other, "Unknown command"))
        }
    }
}

named!(parse_command<(&str, &str)>, do_parse!(
    command: map_res!(take_while!(is_alphabetic), str::from_utf8) >>
    opt!(space) >>
    value: map_res!(take_until!("\n"), str::from_utf8) >>
    (command, value)
));


#[test]
fn test_parse_command() {
    let (_, (command, value)) = parse_command(b"SETDESC desc value\n").unwrap();

    assert_eq!(command, "SETDESC");
    assert_eq!(value, "desc value");

    let (_, (command, value)) = parse_command(b"GETPIN\n").unwrap();
    assert_eq!(command, "GETPIN");
    assert_eq!(value, "");
}
