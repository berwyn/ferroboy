pub fn word_to_u16(word: (u8, u8)) -> u16 {
    u16::from(word.0) << 8 | u16::from(word.1)
}
