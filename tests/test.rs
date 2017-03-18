extern crate ttyaskpass;

use std::io::{ self, Write, Cursor };
use std::sync::mpsc::{ Sender, Receiver, channel };
use ttyaskpass::raw_askpass;


struct FakeTTY(Sender<Vec<u8>>);

impl FakeTTY {
    fn new() -> (Self, Receiver<Vec<u8>>) {
        let (sender, receiver) = channel();
        (FakeTTY(sender), receiver)
    }
}

impl Write for FakeTTY {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.send(buf.into())
            .map(|_| buf.len())
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn test_raw_askpass() {
    let start = [13, 80, 97, 115, 115, 119, 111, 114, 100, 58, 32, 27, 91, 51, 56, 59, 53, 59, 51, 48, 109, 126, 126, 27, 91, 51, 56, 59, 53, 59, 51, 48, 109, 126, 126, 27, 91, 51, 56, 59, 53, 59, 51, 48, 109, 126, 126, 27, 91, 51, 56, 59, 53, 59, 51, 48, 109, 126, 126, 27, 91, 51, 57, 109];
    let end = [13, 80, 97, 115, 115, 119, 111, 114, 100, 58, 32, 27, 91, 51, 56, 59, 53, 59, 50, 48, 56, 109, 126, 126, 27, 91, 51, 56, 59, 53, 59, 50, 50, 109, 126, 126, 27, 91, 51, 56, 59, 53, 59, 49, 52, 49, 109, 126, 126, 27, 91, 51, 56, 59, 53, 59, 57, 48, 109, 126, 126, 27, 91, 51, 57, 109, 27, 91, 50, 75, 13];

    let input = Cursor::new(b"password\n");
    let (fake_tty, recv) = FakeTTY::new();
    let password = raw_askpass::<Vec<u8>, _, _>(input, fake_tty, '~').unwrap();
    assert_eq!(password, b"password");

    let output = recv.iter().collect::<Vec<Vec<u8>>>().concat();
    assert!(output.starts_with(&start));
    assert!(output.ends_with(&end));
}
