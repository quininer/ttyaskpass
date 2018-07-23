use std::io;
use mortal::{ Event, Terminal, PrepareState };
#[cfg(unix)] use mortal::unix::OpenTerminalExt;


pub struct Term {
    pub inner: Terminal,
    state: Option<PrepareState>
}

impl Term {
    pub fn new() -> io::Result<Term> {
        #[cfg(not(unix))]
        let terminal = Terminal::new()?;

        #[cfg(unix)]
        let terminal = Terminal::from_path("/dev/tty")?;

        let state = terminal.prepare(Default::default())?;

        Ok(Term { inner: terminal, state: Some(state) })
    }

    pub fn read_event<F>(&self, mut f: F) -> io::Result<()>
        where F: FnMut(Event) -> io::Result<bool>
    {
        if f(Event::NoEvent)? {
            return Ok(());
        }

        loop {
            if let Some(ev) = self.inner.read_event(None)? {
                if f(ev)? {
                    break
                }
            }
        }

        Ok(())
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        if let Some(state) = self.state.take() {
            let _ = self.inner.restore(state);
        }
    }
}
