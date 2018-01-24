use std::mem;
use std::fs::File;
use std::io::{ self, Read, Write };
use std::os::unix::io::AsRawFd;
use libc::{ termios, tcgetattr, tcsetattr, cfmakeraw };
use termion::get_tty;
use termion::event::Key;
use termion::input::TermRead;


pub struct RawTTY<F: AsRawFd> {
    prev_termios: termios,
    tty: F
}

impl RawTTY<File> {
    pub fn new() -> io::Result<RawTTY<File>> {
        Self::from_tty(get_tty()?)
    }
}

impl<F: AsRawFd> RawTTY<F> {
    pub fn from_tty(tty: F) -> io::Result<RawTTY<F>> {
        unsafe {
            let tty_fd = tty.as_raw_fd();
            let mut ios = mem::zeroed();

            if tcgetattr(tty_fd, &mut ios) != 0 {
                return Err(io::Error::last_os_error())
            }
            let prev_termios = ios;

            cfmakeraw(&mut ios);

            if tcsetattr(tty_fd, 0, &ios) != 0 {
                return Err(io::Error::last_os_error())
            }

            Ok(RawTTY { prev_termios, tty })
        }
    }
}

impl<F: AsRawFd + Read> Read for RawTTY<F> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.tty.read(buf)
    }
}

impl<F: AsRawFd + Write> Write for RawTTY<F> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.tty.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.tty.flush()
    }
}

impl<F: AsRawFd> Drop for RawTTY<F> {
    fn drop(&mut self) {
        unsafe {
            tcsetattr(self.tty.as_raw_fd(), 0, &self.prev_termios);
        }
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

#[cfg_attr(
    any(target_os = "macos", target_os = "ios"),
    should_panic
)]
#[test]
fn test_raw_tty_create() {
    assert!(RawTTY::new().is_ok());
}
