mod readtty;
mod colorhash;

use std::io;
use bstr::ByteSlice;
use crossterm::KeyEvent;
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
        let mut terminal = Term::new()?;
        let mut pos = 0;

        terminal.read_event(|terminal, event| {
            let buf = self.buf.as_mut();
            let star = self.star;

            match event {
                KeyEvent::Enter => return Ok(true),
                KeyEvent::Char(c) => if buf.len() - pos > c.len_utf8() {
                    c.encode_utf8(&mut buf[pos..]);
                    pos += c.len_utf8();
                },
                KeyEvent::Backspace => if let Some((start, ..)) = buf[..pos].char_indices().last() {
                    pos = start;
                },
                KeyEvent::Esc => pos = 0,
                KeyEvent::Ctrl('c') =>
                    return Err(io::Error::new(io::ErrorKind::Interrupted, "Ctrl-c")),
                KeyEvent::Null => (),
                _ => return Ok(false)
            }

            let colors = match pos {
                0 => ColorStar::from(star),
                1..=7 => random_color(star),
                p => hash_chars_as_color(star, &buf[..p])
            };

            write!(terminal, "\r{} ", prompt)?;
            colors.write_to(terminal)?;

            Ok(false)
        })?;

        write!(&mut terminal, "\r")?;
        terminal.clear_current_line()?;

        let buf = self.buf.as_mut();
        Ok(&buf[..pos])
    }
}
