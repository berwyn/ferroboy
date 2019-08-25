use bitflags::bitflags;

/// Selectors for CPU registers
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Register {
    // 8bit
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,

    // Computed 16bit
    AF,
    BC,
    DE,
    HL,

    // 16bit
    SP,
    PC,
}

bitflags! {
    /// Bitflags for the CPU state. The Gameboy's Z80 doesn't use the lower four flags,
    /// so they should always be `0`.
    pub struct Flags: u8 {
        const CLEAR = 0b00000000;
        const CARRY = 0b00010000;
        const HALF_CARRY = 0b00100000;
        const SUBTRACTION = 0b0100000;
        const ZERO = 0b10000000;
    }
}

impl Flags {
    fn reset(&mut self) {
        self.bits = 0;
    }
}

/// An implementation of a Zilog Z80 DMG-01 variant
#[derive(Debug)]
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
        Self {
            halt: false,
            interrupt_mode_enabled: false,

            clock: 0,
            f: Flags::CLEAR,

            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,

            sp: 0,
            pc: 0,
        }
    }

    pub fn reg16_to_reg8(register: Register) -> Result<(Register, Register), String> {
        match register {
            Register::AF => Ok((Register::A, Register::F)),
            Register::BC => Ok((Register::B, Register::C)),
            Register::DE => Ok((Register::D, Register::E)),
            Register::HL => Ok((Register::H, Register::L)),
            _ => Err("Invalid 16bit register pair".into()),
        }
    }

    pub fn get(&self, register: &Register) -> Result<u8, String> {
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

    pub fn set<F>(&mut self, register: &Register, f: F) -> Result<(), String>
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

        Ok(())
    }

    pub fn has_flag(&self, flag: Flags) -> bool {
        self.f & flag == flag
    }

    pub fn set_flag(&mut self, flag: Flags) {
        self.f |= flag
    }

    pub fn clear_flag(&mut self, flag: Flags) {
        self.f -= flag;
    }

    pub fn reset_flags(&mut self) {
        self.f.reset()
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
        let (high, low) = CPU::reg16_to_reg8(Register::AF).unwrap();

        assert_eq!(Register::A, high);
        assert_eq!(Register::F, low);

        let (high, low) = CPU::reg16_to_reg8(Register::BC).unwrap();

        assert_eq!(Register::B, high);
        assert_eq!(Register::C, low);

        let (high, low) = CPU::reg16_to_reg8(Register::DE).unwrap();

        assert_eq!(Register::D, high);
        assert_eq!(Register::E, low);

        let (high, low) = CPU::reg16_to_reg8(Register::HL).unwrap();

        assert_eq!(Register::H, high);
        assert_eq!(Register::L, low);
    }

    #[test]
    fn it_prevents_invalid_16bit_register_conversions() {
        assert!(CPU::reg16_to_reg8(Register::A).is_err());
        assert!(CPU::reg16_to_reg8(Register::B).is_err());
        assert!(CPU::reg16_to_reg8(Register::C).is_err());
        assert!(CPU::reg16_to_reg8(Register::D).is_err());
        assert!(CPU::reg16_to_reg8(Register::E).is_err());
        assert!(CPU::reg16_to_reg8(Register::F).is_err());
        assert!(CPU::reg16_to_reg8(Register::H).is_err());
        assert!(CPU::reg16_to_reg8(Register::L).is_err());

        assert!(CPU::reg16_to_reg8(Register::SP).is_err());
        assert!(CPU::reg16_to_reg8(Register::PC).is_err());
    }
}
