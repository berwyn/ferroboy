use crate::helpers::{u16_to_word, word_to_u16};
use bitflags::bitflags;

/// `Register` is an enum to help indicate which registers
/// an operation should apply to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Register {
    // 8bit
    /// Register A is the 6502's accumulator.
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,

    // Computed 16bit
    /// Register AF uses the Accumulator as the high byte and
    /// the flags as the low byte, creating a pseudo-16bit register.
    AF,
    /// Register BC uses Register B as the high byte and register
    /// C as the low byte, creating a pseudo-16bit register.
    BC,
    /// Register DE uses Register D as the high byte and register
    /// E as the low byte, creating a pseudo-16bit register.
    DE,
    /// Register HL uses Register H as the high byte and register
    /// L as the low byte, creating a pseudo-16bit register.
    HL,

    // 16bit
    /// The stack pointer
    SP,
    /// The program counter
    PC,
}

impl Register {
    /// Registers AF, BC, DE, and HL aren't actually registers but instead
    /// use two registers to create a pseuo-16bit register. This function will
    /// take those pseudo-16bit register selectors and return the appropriate
    /// (high, low) selector tuple to index into the 8bit registers they use.
    pub fn to_8bit_pair(self) -> Result<(Register, Register), String> {
        match self {
            Register::AF => Ok((Register::A, Register::F)),
            Register::BC => Ok((Register::B, Register::C)),
            Register::DE => Ok((Register::D, Register::E)),
            Register::HL => Ok((Register::H, Register::L)),
            _ => Err("Invalid 16bit register pair".into()),
        }
    }
}

bitflags! {
    /// Bitflags for the CPU state. The Gameboy's Z80 doesn't use the lower four flags,
    /// so they should always be `0`.
    pub struct Flags: u8 {
        const CLEAR = 0b0000_0000;
        const CARRY = 0b0001_0000;
        const HALF_CARRY = 0b0010_0000;
        const SUBTRACTION = 0b0100_0000;
        const ZERO = 0b1000_0000;
    }
}

impl Default for Flags {
    fn default() -> Self {
        Flags::CLEAR
    }
}

/// An implementation of a Zilog Z80 DMG-01 variant
#[derive(Debug, Default)]
pub struct CPU {
    halt: bool,
    interrupt_mode_enabled: bool,

    clock: u64,
    f: Flags,

    // 8-bit
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,

    // 16-bit
    sp: u16,
    pc: u16,
}

impl CPU {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get(&self, register: Register) -> Result<u8, String> {
        let selected = match register {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::H => self.h,
            Register::L => self.l,
            _ => return Err("Invalid register".into()),
        };

        Ok(selected)
    }

    pub fn get16(&self, register: Register) -> Result<u16, String> {
        let selected = match register {
            Register::SP => self.sp,
            Register::PC => self.pc,
            Register::AF | Register::BC | Register::DE | Register::HL => {
                let (high, low) = register.to_8bit_pair()?;
                let (high, low) = (self.get(high)?, self.get(low)?);

                word_to_u16((high, low))
            }
            _ => return Err("Invalid register".into()),
        };

        Ok(selected)
    }

    pub fn set(&mut self, register: Register, value: u8) -> Result<u8, String> {
        self.mutate(register, |_| value)
    }

    pub fn mutate<F>(&mut self, register: Register, f: F) -> Result<u8, String>
    where
        F: FnOnce(&u8) -> u8,
    {
        let selected = match register {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
            Register::C => &mut self.c,
            Register::D => &mut self.d,
            Register::E => &mut self.e,
            Register::H => &mut self.h,
            Register::L => &mut self.l,
            _ => return Err("Can't use 16bit registers".into()),
        };

        *selected = f(selected);

        Ok(*selected)
    }

    pub fn set16(&mut self, register: Register, value: u16) -> Result<u16, String> {
        self.mutate16(register, |_| value)
    }

    pub fn mutate16<F>(&mut self, register: Register, f: F) -> Result<u16, String>
    where
        F: FnOnce(&u16) -> u16,
    {
        let selected = match register {
            Register::SP => {
                self.sp = f(&self.sp);
                self.sp
            }
            Register::PC => {
                self.pc = f(&self.pc);
                self.pc
            }
            Register::AF | Register::BC | Register::DE | Register::HL => {
                let word = self.get16(register)?;
                let word = f(&word);

                let (high_byte, low_byte) = u16_to_word(word);
                let (high, low) = register.to_8bit_pair()?;

                self.set(high, high_byte)?;
                self.set(low, low_byte)?;

                word
            }
            _ => return Err("Invalid register".into()),
        };

        Ok(selected)
    }

    // FIXME: Remove this annotation after implementing RET and friends
    #[allow(dead_code)]
    pub fn has_flag(&self, flag: Flags) -> bool {
        self.f & flag == flag
    }

    pub fn set_flag(&mut self, flag: Flags) {
        self.f |= flag
    }

    pub fn clear_flag(&mut self, flag: Flags) {
        self.f -= flag;
    }

    pub fn set_clock<F>(&mut self, f: F)
    where
        F: FnOnce(&u64) -> u64,
    {
        self.clock = f(&self.clock)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::State;

    #[test]
    fn it_checks_if_a_flag_is_set() {
        let mut state = State::new();
        state.cpu.f = Flags::ZERO | Flags::HALF_CARRY;

        assert!(state.cpu.has_flag(Flags::ZERO));
        assert!(state.cpu.has_flag(Flags::HALF_CARRY));
    }

    #[test]
    fn it_sets_a_flag() {
        let mut state = State::new();
        state.cpu.f = Flags::ZERO;

        assert_eq!(Flags::ZERO, state.cpu.f);

        state.cpu.set_flag(Flags::CARRY);

        assert_eq!(Flags::ZERO | Flags::CARRY, state.cpu.f);
    }

    #[test]
    fn it_clears_a_flag() {
        let mut state = State::new();
        state.cpu.f = Flags::CARRY | Flags::HALF_CARRY;

        state.cpu.clear_flag(Flags::CARRY);

        assert_eq!(Flags::HALF_CARRY, state.cpu.f);
    }

    #[test]
    fn it_converts_16bit_register_to_8bit_pairs() {
        let (high, low) = Register::AF.to_8bit_pair().unwrap();

        assert_eq!(Register::A, high);
        assert_eq!(Register::F, low);

        let (high, low) = Register::BC.to_8bit_pair().unwrap();

        assert_eq!(Register::B, high);
        assert_eq!(Register::C, low);

        let (high, low) = Register::DE.to_8bit_pair().unwrap();

        assert_eq!(Register::D, high);
        assert_eq!(Register::E, low);

        let (high, low) = Register::HL.to_8bit_pair().unwrap();

        assert_eq!(Register::H, high);
        assert_eq!(Register::L, low);
    }

    #[test]
    fn it_prevents_invalid_16bit_register_conversions() {
        assert!(Register::A.to_8bit_pair().is_err());
        assert!(Register::B.to_8bit_pair().is_err());
        assert!(Register::C.to_8bit_pair().is_err());
        assert!(Register::D.to_8bit_pair().is_err());
        assert!(Register::E.to_8bit_pair().is_err());
        assert!(Register::F.to_8bit_pair().is_err());
        assert!(Register::H.to_8bit_pair().is_err());
        assert!(Register::L.to_8bit_pair().is_err());

        assert!(Register::SP.to_8bit_pair().is_err());
        assert!(Register::PC.to_8bit_pair().is_err());
    }
}
