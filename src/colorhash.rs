//! Like Chroma-Hash, but with ANSI terminal colors.
//!
//! fork from [myfreeweb/colorhash256](https://github.com/myfreeweb/colorhash256)

use sha3::{ Digest, Sha3_256 };
use termion::color::AnsiValue;

/// Hashes given bytes and encodes the result as ANSI terminal colors.
pub fn hash_as_ansi(bytes: &[u8]) -> [AnsiValue; 8] {
    let mut colors = [AnsiValue(0); 8];

    let mut sha3 = Sha3_256::default();
    sha3.input(bytes);
    let hash = sha3.result();

    for i in 0..8 {
        colors[i] = AnsiValue(16 + (hash[i] % 216));
    }
    colors
}

/// Hashes given chars and encodes the result as ANSI terminal colors.
pub fn hash_chars_as_ansi(chars: &[char]) -> [AnsiValue; 8] {
    let mut buf = [0; 4];
    let mut colors = [AnsiValue(0); 8];
    let mut sha3 = Sha3_256::default();

    for c in chars {
        sha3.input(c.encode_utf8(&mut buf).as_bytes());
    }

    let hash = sha3.result();

    for i in 0..8 {
        colors[i] = AnsiValue(16 + (hash[i] % 216));
    }
    colors
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ansi() {
        assert_eq!(
            hash_as_ansi(b"Correct Horse Battery Staple").iter().map(|v| v.0).collect::<Vec<u8>>(),
            [79, 131, 17, 194, 196, 52, 19, 54]
        );
    }

    #[test]
    fn test_chars_ansi() {
        assert_eq!(
            hash_as_ansi(b"Correct Horse Battery Staple").iter().map(|v| v.0).collect::<Vec<u8>>(),
            hash_chars_as_ansi(&"Correct Horse Battery Staple".chars().collect::<Vec<char>>()).iter().map(|v| v.0).collect::<Vec<u8>>()
        );
    }
}
