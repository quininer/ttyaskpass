extern crate libc;
extern crate rand;
extern crate seckey;
extern crate termion;
extern crate tiny_keccak;

#[macro_use] mod utils;
mod readtty;
mod colorhash;

use std::io::{ self, Read, Write };
use std::iter::repeat;
use rand::random;
use seckey::SecKey;
use termion::clear;
use termion::get_tty;
use termion::event::Key;
use termion::color::{ Fg, Reset, AnsiValue };
use colorhash::{ hash_as_ansi, hash_chars_as_ansi };
use readtty::{ RawTTY, read_from_tty };


/// askpass
///
/// ### Fail When:
/// - IO Error
/// - User Interrupted
/// - RawTTY create fail
/// - SecKey malloc fail
#[inline]
pub fn askpass<T>(prompt: &str, star: char) -> io::Result<T>
    where T: From<Vec<u8>>
{
    raw_askpass(RawTTY::new()?, get_tty()?, prompt, star)
}

pub fn raw_askpass<T, I, O>(input: I, mut output: O, prompt: &str, star: char) -> io::Result<T>
    where
        T: From<Vec<u8>>,
        I: Read,
        O: Write
{
    let mut pos = 0;
    let mut buf = SecKey::new([char::default(); 256])
        .map_err(|_| err!(Other, "SecKey malloc fail"))?;

    read_from_tty(input, |key| {
        let mut buf = buf.write();
        match key {
            Key::Char('\n') => return Ok(true),
            Key::Char(c) => if pos < buf.len() {
                buf[pos] = c;
                pos += 1;
            },
            Key::Backspace | Key::Delete if pos >= 1 => pos -= 1,
            Key::Ctrl('c') => return Err(err!(Interrupted, "Ctrl-c")),
            _ => ()
        }

        let colors = match pos {
            0 => [AnsiValue(30); 8],
            1...7 => hash_as_ansi(&[random()]),
            p => hash_chars_as_ansi(&buf[..p])
        };

        write!(
            output,
            "\r{} {}{}",
            prompt,
            repeat(star)
                .take(4)
                .zip(&colors[..4])
                .map(|(star, &color)| format!("{}{}{}", Fg(color), star, star))
                .collect::<String>(),
            Fg(Reset)
        )?;
        Ok(false)
    })?;

    write!(output, "{}\r", clear::CurrentLine)?;
    let output = buf.read().iter().take(pos).collect::<String>();
    Ok(T::from(output.into_bytes()))
}
