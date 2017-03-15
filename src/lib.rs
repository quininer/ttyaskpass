extern crate rand;
extern crate tiny_keccak;
extern crate termion;
extern crate seckey;

#[macro_use] mod utils;
mod readtty;
pub mod colorhash;

use std::io::{ self, Write };
use std::iter::repeat;
use rand::random;
use seckey::SecKey;
use termion::get_tty;
use termion::event::Key;
use termion::color::{ Fg, Reset, AnsiValue };
use readtty::read_from_tty;
use colorhash::{ hash_as_ansi, hash_chars_as_ansi };


pub fn askpass<T>(star: char) -> io::Result<T>
    where T: From<Vec<u8>>
{
    let mut pos = 0;
    let mut buf: SecKey<[char; 256]> = SecKey::new([Default::default(); 256])
        .map_err(|_| err!(Other, "SecKey malloc fail"))?;
    let mut tty = get_tty()?;

    read_from_tty(|key| {
        let mut buf = buf.write();
        match key {
            Key::Char('\n') => return Ok(true),
            Key::Char(c) => if pos < buf.len() {
                buf[pos] = c;
                pos += 1;
            },
            Key::Backspace | Key::Delete if pos >= 1 => pos -= 1,
            Key::Ctrl('c') => return Err(err!(ConnectionAborted)),
            _ => (),
        }

        let colors = match pos {
            0 => [AnsiValue(30); 8],
            1...7 => hash_as_ansi(&[random(); 16]),
            _ => hash_chars_as_ansi(&buf[..pos])
        };

        write!(
            tty,
            "\rPassword: {}{}",
            repeat(star)
                .take(4)
                .zip(&colors[..4])
                .map(|(star, &color)| format!("{}{}{}", Fg(color), star, star))
                .collect::<String>(),
            Fg(Reset)
        )?;
        Ok(false)
    })?;

    write!(tty, "\r{:<18}\r", "")?;
    let output = buf.read()[..pos].iter().collect::<String>();
    Ok(T::from(output.into_bytes()))
}
