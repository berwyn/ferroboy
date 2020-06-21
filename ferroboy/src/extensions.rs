/// Used to check if an ALU operation would trigger a half-carry.
///
/// The DMG-01 used a 4-bit ALU, meaning that operations were
/// done nybble-by-nybble, leading to a carry flag for each nybble.
/// Because Ferroboy needs to check this on multiple types, helper
/// functions become bulky, so we've wrapped this up into a trait.
#[deprecated = "Replaced with ALU trait"]
pub trait HalfCarry {
    fn half_carry(&self, other: Self) -> bool;
}

impl HalfCarry for u8 {
    fn half_carry(&self, other: Self) -> bool {
        ((self & 0xF) + (other & 0xF)) & 0x10 == 0x10
    }
}

impl HalfCarry for u16 {
    fn half_carry(&self, other: Self) -> bool {
        ((self & 0xFFF) + (other & 0xFFF)) & 0x1000 == 0x1000
    }
}

/// Used to check if an ALU operation would trigger a carry.
///
/// The DMG-01 used a 4-bit ALU, meaning that operations were
/// done nybble-by-nybble, leading to a carry flag for each nybble.
/// Because Ferroboy needs to check this on multiple types, helper
/// functions became bulky, so we've wrapped this up into a trait.
#[deprecated = "Replaced with ALU trait"]
pub trait Carry {
    fn carry(&self, other: Self) -> bool;
}

impl Carry for u8 {
    fn carry(&self, other: Self) -> bool {
        self.checked_add(other).is_none()
    }
}

impl Carry for u16 {
    fn carry(&self, other: Self) -> bool {
        self.checked_add(other).is_none()
    }
}

#[cfg(test)]
mod tests {
    mod half_carry {
        use crate::extensions::HalfCarry;

        mod for_u8 {
            use super::*;

            #[test]
            fn it_checks_explicit_carry() {
                assert!(0xFu8.half_carry(0xF));
            }

            #[test]
            fn it_checks_implicit_carry() {
                assert!(0x4u8.half_carry(0xF));
            }

            #[test]
            fn it_checks_non_carry() {
                assert!(!0x4u8.half_carry(0x3));
            }
        }

        mod for_u16 {
            use super::*;

            #[test]
            fn it_checks_explicit_carry() {
                assert!(0xF00u16.half_carry(0xF00));
            }

            #[test]
            fn it_checks_implicit_carry() {
                assert!(0x400u16.half_carry(0xF00));
            }

            #[test]
            fn it_checks_non_carry() {
                assert!(!0x400u16.half_carry(0x300));
            }
        }
    }

    mod carry {
        use crate::extensions::Carry;

        mod for_u8 {
            use super::*;

            #[test]
            fn it_checks_explicit_carry() {
                assert!(0xF0u8.carry(0xF0));
            }

            #[test]
            fn it_checks_implicit_carry() {
                assert!(0x40u8.carry(0xC0));
            }

            #[test]
            fn it_checks_non_carry() {
                assert!(!0x80u8.carry(0x40));
            }

            #[test]
            fn it_checks_simple_overflows() {
                assert!(0xFFu8.carry(0x01));
            }
        }

        mod for_u16 {
            use super::*;

            #[test]
            fn it_checks_explicit_carry() {
                assert!(0x8000u16.carry(0x8000));
            }

            #[test]
            fn it_checks_implicit_carry() {
                assert!(0x4000u16.carry(0xC000));
            }

            #[test]
            fn it_checks_non_carry() {
                assert!(!0x8000u16.carry(0x4000));
            }

            #[test]
            fn it_checks_simple_overflows() {
                assert!(0xFFFFu16.carry(0x0001));
            }
        }
    }
}
