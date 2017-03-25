#![cfg(feature = "pinentry")]

use std::str;
use std::borrow::Cow;
use std::time::Duration;
use std::io::{ self, Write };
use nom::{ space, is_alphabetic };
use termion::get_tty;
use termion::event::Key;
use termion::color::{ Fg, Red, Reset };
use seckey::Bytes;
use super::readtty::{ RawTTY, read_from_tty };
use super::raw_askpass;


#[macro_export]
macro_rules! dump {
    ( PRINT: $tty:expr, $message:expr ) => {
        write!($tty, "{}\n\r", $message)
    };
    ( WARN : $tty:expr, $message:expr ) => {
        write!(
            $tty,
            "\n\r *** {}{}{} ***\n\n\r",
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
    ( OK: $tty:expr, $message:expr ) => {
        writeln!($tty, "OK {}", $message)
    };
    ( ERR : $tty:expr, $message:expr ) => {
        writeln!($tty, "ERR {}", $message)
    };
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
    pub notok: String,
    pub cancel: String,
    pub quality_bar: String,
    pub quality_bar_tt: String,
    pub title: String,
    pub timeout: Option<Duration>
}

impl Pinentry {
    pub fn get_pin(&mut self, output: &mut Write) -> io::Result<()> {
        let (mut intput, mut tty) = (RawTTY::new()?, get_tty()?);
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
            self.error.clear();
        }

        dump!(PRINT: tty, message)?;

        loop {
            pin = raw_askpass(&mut intput, &mut tty, &prompt, '*')
                .map(|p| Bytes::from(p.into_bytes()))?;

            if !self.repeat.is_empty() {
                let repeat_error =
                    if !self.repeat_error.is_empty() { &self.repeat_error }
                    else { "Passphrases don't match." };
                let pin2 = raw_askpass(&mut intput, &mut tty, &self.repeat, '*')
                    .map(|p| Bytes::from(p.into_bytes()))?;

                if pin != pin2 {
                    dump!(WARN: tty, repeat_error)?;
                    continue
                }
            }

            break
        }

        drop((intput, tty));

        if !self.repeat.is_empty() {
            dump!(S: output, "PIN_REPEATED")?;
        }
        dump!(D: output, unsafe { str::from_utf8_unchecked(&pin) })?;
            //              ^- SAFE: because pin from `String`.

        Ok(())
    }

    pub fn confirm(&mut self, any_flag: bool) -> io::Result<Button> {
        let (mut input, mut tty) = (RawTTY::new()?, get_tty()?);

        let message =
            if !self.description.is_empty() { &self.description }
            else if !self.title.is_empty() { &self.title }
            else { "Confirm:" };
        let ok =
            if !self.ok.is_empty() { &self.ok }
            else { "Ok" };
        let ok_button = ok.to_lowercase().chars().next().unwrap();
            //                                              ^- SAFE: because ok is not empty
        let cancel_button = self.cancel.to_lowercase().chars().next();
        let notok_button = self.notok.to_lowercase().chars().next();

        if !self.error.is_empty() {
            dump!(WARN: tty, self.error)?;
            self.error.clear();
        }

        dump!(PRINT: tty, message)?;

        if any_flag {
            dump!(PRINT: tty, format!("{}", ok))?;
            dump!(PRINT: tty, "Press any key to continue.")?;
        } else {
            dump!(PRINT: tty, format!("[{}] {}", ok_button, ok))?;
            if let Some(button) = cancel_button {
                dump!(PRINT: tty, format!("[{}] {}", button, self.cancel))?;
            }
            if let Some(button) = notok_button {
                dump!(PRINT: tty, format!("[{}] {}", button, self.notok))?;
            }
        }

        let mut button = Button::NotOk;
        read_from_tty(&mut input, |key| {
            match key {
                Key::Null => return Ok(false),
                _ if any_flag => button = Button::Ok,
                Key::Char(c) if c == ok_button => button = Button::Ok,
                Key::Char(c) if Some(c) == cancel_button => button = Button::Cancel,
                Key::Char(c) if Some(c) == notok_button => button = Button::NotOk,
                Key::Ctrl('c') => button = Button::Cancel,
                _ => {
                    dump!(PRINT: tty, "Invalid selection.")?;
                    return Ok(false)
                }
            };

            Ok(true)
        })?;

        Ok(button)
    }
}

#[derive(Debug)]
pub enum Button {
    Ok,
    Cancel,
    NotOk
}

named!(pub parse_command<(&str, &str)>, do_parse!(
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
