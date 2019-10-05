use std::{ fmt, io };


pub struct Term {
    _raw: crossterm::RawScreen,
    term: crossterm::Terminal
}

impl Term {
    pub fn new() -> io::Result<Term> {
        let raw = crossterm::RawScreen::into_raw_mode()
            .map_err(map_io_err)?;
        let term = crossterm::Terminal::new();
        Ok(Term { _raw: raw, term })
    }

    pub fn read_event<F>(&mut self, mut f: F) -> io::Result<()>
        where F: FnMut(&mut Self, crossterm::KeyEvent) -> io::Result<bool>
    {
        if f(self, crossterm::KeyEvent::Null)? {
            return Ok(());
        }

        let input = crossterm::input();

        for event in input.read_sync() {
            if let crossterm::InputEvent::Keyboard(event) = event {
                if f(self, event)? {
                    break
                }
            }
        }

        Ok(())
    }

    pub fn write_fmt(&mut self, args: fmt::Arguments) -> io::Result<usize> {
        self.term.write(args).map_err(map_io_err)
    }

    pub fn clear_current_line(&mut self) -> io::Result<()> {
        self.term.clear(crossterm::ClearType::CurrentLine)
            .map_err(map_io_err)
    }
}

pub fn map_io_err(err: crossterm::ErrorKind) -> io::Error {
    match err {
        crossterm::ErrorKind::IoError(err) => err,
        err => io::Error::new(io::ErrorKind::Other, err)
    }
}
