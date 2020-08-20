/// A trait to encapsulate DMG-01 4-bit ALU operations.
pub trait ALU: Copy + Sized {
    /// Adds a value to this object, returning the new value,
    /// whether a carry happened, and whether a half-carry happened.
    fn alu_add(self, other: Self) -> (Self, bool, bool);

    /// Subtracts a value from this object, returning the new value,
    /// whether a carry (borrow) happened, and whether a half-carry
    /// (half-borrow) happened.
    fn alu_sub(self, other: Self) -> (Self, bool, bool);
}

impl ALU for u8 {
    fn alu_add(self, other: Self) -> (Self, bool, bool) {
        let result = self.wrapping_add(other);
        let carry = self > result;
        let half_carry = (self & 0x0F) + (other & 0x0F) >= 0x10;

        (result, carry, half_carry)
    }

    fn alu_sub(self, other: Self) -> (Self, bool, bool) {
        let result = self.wrapping_sub(other);
        let carry = result > self;
        let half_carry = (other & 0x0F) > (self & 0x0F);

        (result, carry, half_carry)
    }
}

impl ALU for u16 {
    fn alu_add(self, other: Self) -> (Self, bool, bool) {
        let result = self.wrapping_add(other);
        let carry = self > result;
        let half_carry = (self & 0x0FFF) + (other & 0x0FFF) >= 0x1000;

        (result, carry, half_carry)
    }

    fn alu_sub(self, other: Self) -> (Self, bool, bool) {
        let result = self.wrapping_sub(other);
        let carry = result > self;
        let half_carry = (other & 0x0FFF) > (self & 0x0FFF);

        (result, carry, half_carry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod u8 {
        use super::*;

        #[test]
        fn it_should_half_carry() {
            assert_eq!((0x10, false, true), 0x0Fu8.alu_add(1));
            assert_eq!((0x0F, false, true), 0x10u8.alu_sub(1));
        }

        #[test]
        fn it_should_carry() {
            assert_eq!((0x00, true, true), 0xFFu8.alu_add(1));
            assert_eq!((0xFF, true, true), 0x00u8.alu_sub(1));
        }
    }

    mod u16 {
        use super::*;

        #[test]
        fn it_should_half_carry() {
            assert_eq!((0x1000, false, true), 0x0FFFu16.alu_add(1));
            assert_eq!((0x0FFF, false, true), 0x1000u16.alu_sub(1));
        }

        #[test]
        fn it_should_carry() {
            assert_eq!((0x0000, true, true), 0xFFFFu16.alu_add(1));
            assert_eq!((0xFFFF, true, true), 0x0000u16.alu_sub(1));
        }
    }
}
