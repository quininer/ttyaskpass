//! Like Chroma-Hash, but with ANSI terminal colors.
//!
//! fork from [myfreeweb/colorhash256](https://github.com/myfreeweb/colorhash256)

use std::io;
use rand::{ Rng, thread_rng };
use sha3::Shake256;
use sha3::digest::{ Input, ExtendableOutput, XofReader };
use mortal::{ Terminal, Color };


pub struct ColorStar {
    colors: [u8; 4],
    star: char
}

impl ColorStar {
    pub fn from(star: char) -> ColorStar {
        ColorStar { colors: [0; 4], star }
    }

    pub fn write(&self, term: &Terminal) -> io::Result<()> {
        fn color(c: u8) -> Color {
            match (c % 7) + 1 {
//                0 => Color::Black,
                1 => Color::Blue,
                2 => Color::Cyan,
                3 => Color::Green,
                4 => Color::Magenta,
                5 => Color::Red,
                6 => Color::White,
                7 => Color::Yellow,
                _ => unreachable!()
            }
        }

        for &c in &self.colors {
            term.set_fg(color(c))?;
            term.write_char(self.star)?;
            term.write_char(self.star)?;
        }

        term.set_fg(None)?;

        Ok(())
    }
}


/// Hashes given chars and encodes the result as ANSI terminal colors.
pub fn hash_chars_as_color(star: char, buf: &mut [u8; 4], chars: &[char]) -> ColorStar {
    let mut colors = [0; 4];
    let mut hasher = Shake256::default();

    for c in chars {
        hasher.input(c.encode_utf8(buf).as_bytes());
    }

    hasher.xof_result().read(&mut colors);

    mask_colors(&mut colors);

    ColorStar { colors, star }
}

pub fn random_color(star: char) -> ColorStar {
    let mut colors = [0; 4];
    thread_rng().fill(&mut colors);
    mask_colors(&mut colors);
    ColorStar { colors, star }
}

fn mask_colors(colors: &mut [u8]) {
    for b in colors {
        *b = 16 + (*b % 216);
    }
}
