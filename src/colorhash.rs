//! Like Chroma-Hash, but with ANSI terminal colors.
//!
//! fork from [myfreeweb/colorhash256](https://github.com/myfreeweb/colorhash256)

use std::io;
use rand::{ Rng, thread_rng };
use sha3::Shake256;
use sha3::digest::{ Input, ExtendableOutput, XofReader };
use crate::readtty::{ map_io_err, Term };


pub struct ColorStar {
    colors: [u8; 4],
    star: char
}

impl ColorStar {
    pub fn from(star: char) -> ColorStar {
        ColorStar { colors: [0; 4], star }
    }

    pub fn write_to(&self, term: &mut Term) -> io::Result<()> {
        let color = crossterm::color();

        for &c in &self.colors {
            color.set_fg(crossterm::Color::AnsiValue(c))
                .map_err(map_io_err)?;
            write!(term, "{0}{0}", self.star)?;
        }

        color.reset().map_err(map_io_err)?;

        Ok(())
    }
}


/// Hashes given chars and encodes the result as ANSI terminal colors.
pub fn hash_chars_as_color(star: char, chars: &[u8]) -> ColorStar {
    let mut colors = [0; 4];
    let mut hasher = Shake256::default();
    hasher.input(chars);
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
