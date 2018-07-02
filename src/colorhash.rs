//! Like Chroma-Hash, but with ANSI terminal colors.
//!
//! fork from [myfreeweb/colorhash256](https://github.com/myfreeweb/colorhash256)

use rand::{ Rng, thread_rng };
use sha3::Shake256;
use digest::{ Input, ExtendableOutput, XofReader };

/// Hashes given chars and encodes the result as ANSI terminal colors.
pub fn hash_chars_as_ansi(buf: &mut [u8; 4], chars: &[char]) -> [u8; 4] {
    let mut colors = [0; 4];
    let mut hasher = Shake256::default();

    for c in chars {
        hasher.process(c.encode_utf8(buf).as_bytes());
    }

    hasher.xof_result().read(&mut colors);

    mask_colors(&mut colors);

    colors
}

pub fn random_ansi() -> [u8; 4] {
    let mut colors = [0; 4];
    thread_rng().fill(&mut colors);
    mask_colors(&mut colors);
    colors
}

fn mask_colors(colors: &mut [u8]) {
    for b in colors {
        *b = 16 + (*b % 216);
    }
}
