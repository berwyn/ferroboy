pub fn word_to_u16(word: (u8, u8)) -> u16 {
    u16::from(word.0) << 8 | u16::from(word.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_u8_pairs_to_u16() {
        assert_eq!(0xBEEF, word_to_u16((0xBE, 0xEF)));
    }
}
