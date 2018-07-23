extern crate libc;
extern crate rand;
extern crate sha3;
extern crate digest;
extern crate mortal;
extern crate seckey;

mod readtty;
mod colorhash;

use std::io;
use mortal::{ Event, Key };
use seckey::{ SecKey, TempKey };
use colorhash::{ ColorStar, hash_chars_as_color, random_color };
use readtty::Term;


/// AskPass
///
/// ### Fail When:
///
/// - IO Error
/// - User Interrupted
/// - Terminal prepare fail
/// - `SecKey` malloc fail
#[inline]
pub fn askpass<E, F, T>(prompt: &str, f: F)
    -> Result<T, E>
    where
        F: FnOnce(&str) -> Result<T, E>,
        E: From<io::Error>
{
    raw_askpass(prompt, '*')
        .map_err(Into::into)
        .and_then(|(buf, pos)| {
            let mut buf = buf.read().iter().take(pos).collect::<String>();
            let buf = TempKey::from(&mut buf as &mut str);
            f(&buf)
        })
}

pub fn raw_askpass(prompt: &str, star: char)
    -> io::Result<(SecKey<[char; 256]>, usize)>
{
    let terminal = Term::new()?;

    let mut pos = 0;
    let mut buf = SecKey::new([char::default(); 256])
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "SecKey malloc fail"))?;

    terminal.read_event(|event| {
        let mut chars_buf = [0; 4];
        let mut chars_buf = TempKey::from(&mut chars_buf);
        let mut buf = buf.write();

        match event {
            Event::Key(Key::Enter) => return Ok(true),
            Event::Key(Key::Char(c)) => if pos < buf.len() {
                buf[pos] = c;
                pos += 1;
            },
            Event::Key(Key::Backspace) | Event::Key(Key::Delete) if pos >= 1 => pos -= 1,
            Event::Key(Key::Ctrl('c')) =>
                return Err(io::Error::new(io::ErrorKind::Interrupted, "Ctrl-c")),
            Event::NoEvent => (),
            _ => return Ok(false)
        }

        let colors = match pos {
            0 => ColorStar::from(star),
            1...7 => random_color(star),
            p => hash_chars_as_color(star, &mut chars_buf, &buf[..p])
        };

        write!(terminal.inner, "\r{} ", prompt)?;
        colors.write(&terminal.inner)?;

        Ok(false)
    })?;

    write!(terminal.inner, "\r")?;
    terminal.inner.clear_to_line_end()?;

    Ok((buf, pos))
}
