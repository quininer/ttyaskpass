mod readtty;
mod colorhash;

use std::io;
use bstr::ByteSlice;
use mortal::{ Event, Key };
use colorhash::{ ColorStar, hash_chars_as_color, random_color };
use readtty::Term;


pub struct AskPass<B> {
    star: char,
    buf: B
}

impl<B: AsMut<[u8]>> AskPass<B> {
    pub fn new(buf: B) -> AskPass<B> {
        AskPass { star: '*', buf }
    }

    pub fn with_star(mut self, star: char) -> AskPass<B> {
        self.star = star;
        self
    }

    pub fn as_buffer(&self) -> &B {
        &self.buf
    }

    pub fn into_buffer(self) -> B {
        self.buf
    }
}

impl<B: AsMut<[u8]>> AskPass<B> {
    /// AskPass
    ///
    /// Note that an error (`io::ErrorKind::Interrupted`) will be returned
    /// when the user interrupts with `Ctrl-c`.
    pub fn askpass<'a>(&'a mut self, prompt: &str) -> io::Result<&'a [u8]> {
        let terminal = Term::new()?;
        let mut pos = 0;

        terminal.read_event(|event| {
            let buf = self.buf.as_mut();
            let star = self.star;

            match event {
                Event::Key(Key::Enter) => return Ok(true),
                Event::Key(Key::Char(c)) => if buf.len() - pos > c.len_utf8() {
                    c.encode_utf8(&mut buf[pos..]);
                    pos += c.len_utf8();
                },
                Event::Key(Key::Backspace) => pos = buf[..pos].char_indices()
                    .last()
                    .map(|(start, ..)| start)
                    .unwrap_or(0)
                ,
                Event::Key(Key::Escape) => pos = 0,
                Event::Key(Key::Ctrl('c')) =>
                    return Err(io::Error::new(io::ErrorKind::Interrupted, "Ctrl-c")),
                Event::NoEvent => (),
                _ => return Ok(false)
            }

            let colors = match pos {
                0 => ColorStar::from(star),
                1..=7 => random_color(star),
                p => hash_chars_as_color(star, &buf[..p])
            };

            write!(terminal.inner, "\r{} ", prompt)?;
            colors.write(&terminal.inner)?;

            Ok(false)
        })?;

        write!(terminal.inner, "\r")?;
        terminal.inner.clear_to_line_end()?;

        let buf = self.buf.as_mut();
        Ok(&buf[..pos])
    }
}
