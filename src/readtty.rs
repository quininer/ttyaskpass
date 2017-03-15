use std::io::{ self, Read };
use std::fs::File;
use termion::get_tty;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{ IntoRawMode, RawTerminal };


pub struct RawTTY(pub RawTerminal<File>);

impl Read for RawTTY {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}


pub fn read_from_tty<F>(mut f: F) -> io::Result<()>
    where F: FnMut(Key) -> io::Result<bool>
{
    let raw_tty = RawTTY(get_tty()?.into_raw_mode()?);

    for key in Some(Ok(Key::Null)).into_iter()
        .chain(raw_tty.keys())
    {
        if f(key?)? {
            break
        }
    }

    Ok(())
}
