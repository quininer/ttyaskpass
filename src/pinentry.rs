#![cfg(feature = "pinentry")]

use std::{ io, str };
use std::time::Duration;
use nom::{ space, is_alphabetic };


#[derive(Debug, Clone, Default)]
pub struct Pinentry {
    pub description: Option<String>,
    pub prompt: Option<String>,
    pub key_info: Option<String>,
    pub repeat: Option<String>,
    pub repeat_error: Option<String>,
    pub error: Option<String>,
    pub ok: Option<String>,
    pub not_ok: Option<String>,
    pub cancel: Option<String>,
    pub quality_bar: Option<String>,
    pub quality_bar_tt: Option<String>,
    pub title: Option<String>,
    pub timeout: Option<Duration>
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
    GetInfo,
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
            "GETINFO" => Ok(Command::GetInfo),
            "SETTITLE" => Ok(Command::SetTitle(value.into())),
            "SETTIMEOUT" => Ok(Command::SetTimeout(Duration::from_secs(value.parse().map_err(|err| err!(Other, err))?))),
            "CLEARPASSPHRASE" => Ok(Command::ClearPassphrase),
            _ => Err(err!(Other, "unkown command"))
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
