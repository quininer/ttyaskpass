extern crate rand;
extern crate ansi_term;
extern crate interactor;
extern crate colorhash256;

use std::io::Write;
use std::iter::repeat;
use rand::random;
use ansi_term::{ ANSIStrings, ANSIString };
use ansi_term::Colour::Fixed;
use colorhash256::hash_as_ansi;


pub fn askpass<T: From<Vec<u8>>>(star: char) -> T {
    let star = repeat(star)
        .take(8)
        .map(|c| c as u8)
        .collect::<Vec<u8>>();

    interactor::read_from_tty(|buf, b, tty| {
        if b == 4 {
            write!(tty, "\r{:<18}\r", "").unwrap();
            return ();
        }

        let colors = match buf.len() {
            0 => [30; 8],
            1...7 => hash_as_ansi(&[random(); 4]),
            _ => hash_as_ansi(buf)
        };

        write!(
            tty,
            "\rPassword: {}",
            ANSIStrings(
                &star
                    .chunks(2)
                    .map(String::from_utf8_lossy)
                    .zip(&colors[..4])
                    .map(|(s, &c)| Fixed(c as u8).paint(s))
                    .collect::<Vec<ANSIString>>()
            )
        ).unwrap();
    }, true, true).unwrap().into()
}
