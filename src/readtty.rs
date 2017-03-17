use std::io::{ self, Read };
use std::fs::File;
use termion::get_tty;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{ IntoRawMode, RawTerminal };


/// RawTTY wrapper.
pub struct RawTTY(RawTerminal<File>);

impl RawTTY {
    pub fn new() -> io::Result<RawTTY> {
        Ok(RawTTY(get_tty()?.into_raw_mode()?))
    }
}

impl Read for RawTTY {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

pub fn read_from_tty<T, F>(raw_tty: T, mut f: F) -> io::Result<()>
    where
        T: Read,
        F: FnMut(Key) -> io::Result<bool>
{
    for key in Some(Ok(Key::Null)).into_iter()
        .chain(raw_tty.keys())
    {
        if f(key?)? {
            break
        }
    }

    Ok(())
}
