extern crate rand;
extern crate secstr;
extern crate ansi_term;
extern crate interactor;
extern crate colorhash256;

use std::io::Write;
use rand::random;
use secstr::SecStr;
use ansi_term::{ ANSIStrings, ANSIString };
use ansi_term::Colour::Fixed;
use colorhash256::hash_as_ansi;


pub fn askpass(star: &[u8]) -> SecStr {
    let star = if star.len() == 0 { b"~" } else { star };
    let star = star.iter().cycle().take(8).cloned().collect::<Vec<u8>>();
    interactor::read_from_tty(|buf, b, tty| {
        if b == 4 {
            tty.write(b"\r                                \r").unwrap();
            return;
        };
        let colors = match buf.len() {
            0 => [30; 8],
            1...7 => hash_as_ansi(&[random(); 8]),
            _ => hash_as_ansi(buf)
        };
        let color_string = format!(
            "\rPassword: {}",
            ANSIStrings(
                &star
                    .chunks(2)
                    .zip(&colors[..4])
                    .map(|(s, &c)| Fixed(c as u8).paint(String::from_utf8_lossy(s)))
                    .collect::<Vec<ANSIString>>()
            )
        ).into_bytes();
        tty.write(&color_string).unwrap();
    }, true, true).unwrap().into()
}
