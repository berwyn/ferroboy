/// Because the Gameboy stores 16-bit integers as (high, low)
/// pairs, this function helps to convert those to u16 for easy
/// use in Rust code.
///
/// # Arguments
///
/// * `word` - A (high, low) u8 tuple
pub fn word_to_u16(word: (u8, u8)) -> u16 {
    u16::from(word.0) << 8 | u16::from(word.1)
}

/// Because the Gameboy stores 16-bit integers as (high, low)
/// pairs, this function helps convert a u16 into such a pair
/// to easily store it back into the registers.
///
/// # Arguments
///
/// * `word` - A u16 to convert to a (u8, u8) pair
pub fn u16_to_word(word: u16) -> (u8, u8) {
    let high = (word >> 8) as u8;
    let low = word as u8;

    (high, low)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod word_to_u16 {
        use super::*;

        #[test]
        fn it_converts_u8_pairs_to_u16() {
            assert_eq!(0xBEEF, word_to_u16((0xBE, 0xEF)));
        }
    }

    mod u16_to_word {
        use super::*;

        #[test]
        fn it_converts_u16_to_u8_pairs() {
            assert_eq!((0xBE, 0xEF), u16_to_word(0xBEEF));
        }
    }
}
