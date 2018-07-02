extern crate libc;
extern crate rand;
extern crate sha3;
extern crate digest;
extern crate termion;
extern crate seckey;

pub mod readtty;
pub mod colorhash;

use std::io::{ self, Read, Write };
use std::iter::repeat;
use seckey::{ SecKey, TempKey };
use termion::{ clear, get_tty };
use termion::event::Key;
use termion::color::{ Fg, Reset, AnsiValue };
use colorhash::{ hash_chars_as_ansi, random_ansi };
use readtty::{ RawTTY, read_from_tty };


/// Askpass
///
/// ### Fail When:
///
/// - IO Error
/// - User Interrupted
/// - `RawTTY` create fail
/// - `SecKey` malloc fail
#[inline]
pub fn askpass<F, T>(prompt: &str, f: F)
    -> io::Result<T>
    where F: FnOnce(&str) -> io::Result<T>
{
    raw_askpass(&mut RawTTY::new()?, &mut get_tty()?, prompt, '*')
        .and_then(|(buf, pos)| {
            let mut buf = buf.read().iter().take(pos).collect::<String>();
            let buf = TempKey::from(&mut buf as &mut str);
            f(&buf)
        })
}

pub fn raw_askpass(input: &mut Read, output: &mut Write, prompt: &str, star: char)
    -> io::Result<(SecKey<[char; 256]>, usize)>
{
    let mut pos = 0;
    let mut buf = SecKey::new([char::default(); 256])
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "SecKey malloc fail"))?;

    read_from_tty(input, |key| {
        let mut chars_buf = [0; 4];
        let mut chars_buf = TempKey::from(&mut chars_buf);
        let mut buf = buf.write();

        match key {
            Key::Char('\n') => return Ok(true),
            Key::Char(c) => if pos < buf.len() {
                buf[pos] = c;
                pos += 1;
            },
            Key::Backspace | Key::Delete if pos >= 1 => pos -= 1,
            Key::Ctrl('c') => return Err(io::Error::new(io::ErrorKind::Interrupted, "Ctrl-c")),
            Key::Null => (),
            _ => return Ok(false)
        }

        let colors = match pos {
            0 => [30; 4],
            1...7 => random_ansi(),
            p => hash_chars_as_ansi(&mut chars_buf, &buf[..p])
        };

        write!(
            output,
            "\r{} {}{}",
            prompt,
            repeat(star)
                .take(4)
                .zip(&colors)
                .map(|(star, &color)| format!("{}{}{}", Fg(AnsiValue(color)), star, star))
                .collect::<String>(),
            Fg(Reset)
        )?;

        Ok(false)
    })?;

    write!(output, "{}\r", clear::CurrentLine)?;

    Ok((buf, pos))
}
