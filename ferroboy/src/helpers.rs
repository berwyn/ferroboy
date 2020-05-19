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

/// The DMG-01 uses a 4-bit ALU, so 8-bit operations require two ALU
/// operations, and as such does a carry-check on each nybble. The
/// half-carry flag is set when a value is carried from bit 3 to bit 4.
/// Rust doesn't give us a u4 type, so we'll use this function to check
/// the half-carry on the u8 instead.
pub(crate) fn check_half_carry(left_hand: u8, right_hand: u8) -> bool {
    (left_hand & 0xF) + (right_hand & 0xF) & 0x10 == 0x10
}

/// Similar to the half-carry, because the DMG-01 uses a 4-bit ALU
/// we need to check whether a bit it carried from bit 7 to bit 8
/// so we can set the appropriate flags.
pub(crate) fn check_carry(left_hand: u8, right_hand: u8) -> bool {
    let half_carried = check_half_carry(left_hand, right_hand) as u8;

    let lhs = u16::from(left_hand & 0xF0);
    let rhs = u16::from(right_hand & 0xF0);
    let add = u16::from(half_carried << 4);

    lhs + rhs + add & 0x100 == 0x100
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

    mod check_half_carry {
        use super::*;

        #[test]
        fn it_checks_explicit_carry() {
            assert!(check_half_carry(0b0011_1110, 0b0000_1111));
        }

        #[test]
        fn it_checks_implicit_carry() {
            assert!(check_half_carry(0b0010_0111, 0b0000_1111));
        }

        #[test]
        fn it_checks_non_carry() {
            assert!(!check_half_carry(0b0010_0011, 0b0000_0111));
        }
    }

    mod check_carry {
        use super::*;

        #[test]
        fn it_checks_explicit_carry() {
            assert!(check_carry(0b1000_0000, 0b1000_0000));
        }

        #[test]
        fn it_checks_implicit_carry() {
            assert!(check_carry(0b1100_0000, 0b0110_0000));
        }

        #[test]
        fn it_checks_non_carry() {
            assert!(!check_carry(0b0010_0011, 0b0000_0111));
        }
    }
}
