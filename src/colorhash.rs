//! Like Chroma-Hash, but with ANSI terminal colors.
//!
//! fork from [https://github.com/myfreeweb/colorhash256]

use tiny_keccak::Keccak;
use termion::color::AnsiValue;

/// Hashes given bytes and encodes the result as ANSI terminal colors.
pub fn hash_as_ansi(bytes: &[u8]) -> [AnsiValue; 8] {
    let mut colors = [AnsiValue(0); 8];
    let mut hash = [0; 8];

    let mut sha3 = Keccak::new_sha3_256();
    sha3.update(bytes);
    sha3.finalize(&mut hash);

    for i in 0..8 {
        colors[i] = AnsiValue(16 + (hash[i] % 216));
    }
    colors
}

/// Hashes given chars and encodes the result as ANSI terminal colors.
pub fn hash_chars_as_ansi(chars: &[char]) -> [AnsiValue; 8] {
    let mut buf = [0; 4];
    let mut hash = [0; 8];
    let mut colors = [AnsiValue(0); 8];
    let mut sha3 = Keccak::new_sha3_256();

    for c in chars {
        sha3.update(c.encode_utf8(&mut buf).as_bytes());
    }

    sha3.finalize(&mut hash);

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