use core::convert::TryInto;

use bitflags::bitflags;

use super::register::{Register, WideRegister};
use crate::helpers::{u16_to_word, word_to_u16};

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

/// An implementation of the Gameboy's LR35902 CPU.
#[derive(Debug)]
pub struct CPU {
    halted: bool,
    interrupts_enabled: bool,
    /// Whether or not the previous operation set the CPU to use the CB prefix operations.
    pub(crate) in_prefix_mode: bool,

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
    pub(crate) fn get(&self, register: Register) -> u8 {
        match register {
            Register::A => self.a,
            Register::F => self.f.bits,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::H => self.h,
            Register::L => self.l,
        }
    }

    pub(crate) fn get16(&self, register: WideRegister) -> u16 {
        match register {
            WideRegister::SP => self.sp,
            WideRegister::PC => self.pc,
            WideRegister::AF | WideRegister::BC | WideRegister::DE | WideRegister::HL => {
                let (high, low) = register.try_into().unwrap();
                word_to_u16((self.get(high), self.get(low)))
            }
        }
    }

    pub(crate) fn set(&mut self, register: Register, value: u8) -> u8 {
        let selected = match register {
            Register::A => &mut self.a,
            Register::F => &mut self.f.bits,
            Register::B => &mut self.b,
            Register::C => &mut self.c,
            Register::D => &mut self.d,
            Register::E => &mut self.e,
            Register::H => &mut self.h,
            Register::L => &mut self.l,
        };

        *selected = value;
        *selected
    }

    pub(crate) fn set16(&mut self, register: WideRegister, value: u16) -> u16 {
        match register {
            WideRegister::SP => {
                self.sp = value;
                self.sp
            }
            WideRegister::PC => {
                self.pc = value;
                self.pc
            }
            WideRegister::AF | WideRegister::BC | WideRegister::DE | WideRegister::HL => {
                let (high_byte, low_byte) = u16_to_word(value);
                let (high, low) = register.try_into().unwrap();

                self.set(high, high_byte);
                self.set(low, low_byte);

                self.get16(register)
            }
        }
    }

    pub(crate) fn has_flag(&self, flag: Flags) -> bool {
        self.f & flag == flag
    }

    pub(crate) fn set_flag(&mut self, flag: Flags) {
        self.f |= flag
    }

    pub(crate) fn clear_flag(&mut self, flag: Flags) {
        self.f -= flag;
    }

    pub(crate) fn set_flag_value(&mut self, flag: Flags, value: bool) {
        if value {
            self.f |= flag;
        } else {
            self.f -= flag;
        }
    }

    pub(crate) fn increment_clock(&mut self, amount: u64) {
        self.clock += amount;
    }

    // FIXME: Should this be test-only?
    #[allow(unused)]
    pub(crate) fn set_clock<F>(&mut self, f: F)
    where
        F: FnOnce(&u64) -> u64,
    {
        self.clock = f(&self.clock)
    }

    #[allow(unused)]
    pub(crate) fn interrupts_enabled(&self) -> bool {
        self.interrupts_enabled
    }

    pub(crate) fn enable_interrupts(&mut self) {
        self.interrupts_enabled = true;
    }

    pub(crate) fn disable_interrupts(&mut self) {
        self.interrupts_enabled = false;
    }

    pub(crate) fn is_halted(&self) -> bool {
        self.halted
    }

    pub(crate) fn halt(&mut self) -> bool {
        self.halted = true;
        self.halted
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            halted: false,
            interrupts_enabled: true,
            in_prefix_mode: false,

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::State;

    #[test]
    fn it_checks_if_a_flag_is_set() {
        let mut state = State::default();
        state.cpu.f = Flags::ZERO | Flags::HALF_CARRY;

        assert!(state.cpu.has_flag(Flags::ZERO));
        assert!(state.cpu.has_flag(Flags::HALF_CARRY));
    }

    #[test]
    fn it_sets_a_flag() {
        let mut state = State::default();
        state.cpu.f = Flags::ZERO;

        assert_eq!(Flags::ZERO, state.cpu.f);

        state.cpu.set_flag(Flags::CARRY);

        assert_eq!(Flags::ZERO | Flags::CARRY, state.cpu.f);
    }

    #[test]
    fn it_clears_a_flag() {
        let mut state = State::default();
        state.cpu.f = Flags::CARRY | Flags::HALF_CARRY;

        state.cpu.clear_flag(Flags::CARRY);

        assert_eq!(Flags::HALF_CARRY, state.cpu.f);
    }
}
