use crate::operations::Operation;
use crate::state::State;
use crate::system::{Flags, Register};

// FIXME: This is probably better as (WideRegister)
/*
 * 0x09 is ADD HL,BC
 * 0x19 is ADD HL,DE
 * 0x29 is ADD HL,BC
 * 0x39 is ADD HL,HL
 * 0x49 is ADD HL,SP
 * Since HL is the only valid target, and all the right-hand args are
 * 16-bit registers, we can probably simplify this and keep all state
 * reads and writes here
 */

/// Adds the contents of one 16-bit register to another
///
/// # Opcode Reference
/// ## Assembly definition
/// ```a
/// ADD HL,SP
/// ```
///
/// ## Runtime
/// | Metric | Size |
/// |:-------|:-----|
/// | Length | 1 |
/// | Cycles | 8 |
///
/// ## Flags
/// | Flag | Value |
/// |:-----|:------|
/// | Zero | Not Affected |
/// | Subtraction | 0 |
/// | Half-Carry | Set |
/// | Carry | Set |
///
/// # Examples
/// ```rs
/// let operation = Add16Operation(Register::HL, Register::SP);
/// operation.act(&mut state).unwrap();
/// ```
///
/// # Errors
/// - The operation may fail if an 8-bit register is provided.
#[derive(Debug)]
pub struct Add16Operation(Register, u16);

impl Operation for Add16Operation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        let (high, low) = self.0.to_8bit_pair()?;

        state.cpu.clear_flag(Flags::SUBTRACTION);

        let lower_arg = self.1 & 0xFF;
        let upper_arg = self.1 >> 8;

        let calculated_lower = u16::from(state.cpu.get(low)?) + lower_arg;
        state.cpu.set(low, calculated_lower as u8)?;

        if calculated_lower / 0xFF > 0 {
            state.cpu.set_flag(Flags::HALF_CARRY);
        }

        let calculated_upper =
            u16::from(state.cpu.get(high)?) + upper_arg + (calculated_lower / 0xFF);

        state.cpu.set(high, calculated_upper as u8)?;

        if calculated_upper / 0xFF > 0 {
            state.cpu.set_flag(Flags::CARRY);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_the_lower_byte() {
        let mut state = State::default();
        let op = Add16Operation(Register::BC, 0x0010);

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());
        assert_eq!(0x00, state.cpu.get(Register::C).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());
        assert_eq!(0x10, state.cpu.get(Register::C).unwrap());
        assert!(!state.cpu.has_flag(Flags::HALF_CARRY));
        assert!(!state.cpu.has_flag(Flags::CARRY));
    }

    #[test]
    fn it_adds_the_upper_byte() {
        let mut state = State::default();
        let op = Add16Operation(Register::BC, 0x0F10);

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());
        assert_eq!(0x00, state.cpu.get(Register::C).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0x0F, state.cpu.get(Register::B).unwrap());
        assert_eq!(0x10, state.cpu.get(Register::C).unwrap());
        assert!(!state.cpu.has_flag(Flags::HALF_CARRY));
        assert!(!state.cpu.has_flag(Flags::CARRY));
    }

    #[test]
    fn it_sets_half_carry() {
        let mut state = State::default();
        let op = Add16Operation(Register::BC, 0x00FF);

        state.cpu.set(Register::C, 1).unwrap();

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());
        assert_eq!(0x01, state.cpu.get(Register::C).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0x01, state.cpu.get(Register::B).unwrap());
        assert_eq!(0x00, state.cpu.get(Register::C).unwrap());
        assert!(state.cpu.has_flag(Flags::HALF_CARRY));
        assert!(!state.cpu.has_flag(Flags::CARRY));
    }

    #[test]
    fn it_sets_carry() {
        let mut state = State::default();
        let op = Add16Operation(Register::BC, 0x0001);

        state.cpu.set(Register::B, 0xFF).unwrap();
        state.cpu.set(Register::C, 0xFF).unwrap();

        assert_eq!(0xFF, state.cpu.get(Register::B).unwrap());
        assert_eq!(0xFF, state.cpu.get(Register::C).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());
        assert_eq!(0x00, state.cpu.get(Register::C).unwrap());
        assert!(state.cpu.has_flag(Flags::HALF_CARRY));
        assert!(state.cpu.has_flag(Flags::CARRY));
    }

    #[test]
    fn it_clears_the_subtraction_flag() {
        let mut state = State::default();
        state.cpu.set_flag(Flags::SUBTRACTION);

        Add16Operation(Register::HL, 0).act(&mut state).unwrap();

        assert_ne!(true, state.cpu.has_flag(Flags::SUBTRACTION));
    }
}
