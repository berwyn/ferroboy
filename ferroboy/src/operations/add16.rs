use crate::helpers::u16_to_word;
use crate::operations::Operation;
use crate::state::State;
use crate::system::{Flags, Register, WideRegister};

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
pub struct Add16Operation(WideRegister);

impl Operation for Add16Operation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        if WideRegister::PC.eq(&self.0) {
            return Err("Cannot use PC in ADD".into());
        }

        state.cpu.clear_flag(Flags::SUBTRACTION);

        let (upper_arg, lower_arg) = u16_to_word(state.cpu.get16(self.0)?);
        let upper_arg = upper_arg as u16;
        let lower_arg = lower_arg as u16;

        let calculated_lower = u16::from(state.cpu.get(Register::L)?) + lower_arg;
        state.cpu.set(Register::L, calculated_lower as u8)?;

        if calculated_lower / 0xFF > 0 {
            state.cpu.set_flag(Flags::HALF_CARRY);
        }

        let calculated_upper =
            u16::from(state.cpu.get(Register::H)?) + upper_arg + (calculated_lower / 0xFF);

        state.cpu.set(Register::H, calculated_upper as u8)?;

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
    fn it_adds_the_lower_byte() -> crate::Result<()> {
        let mut state = State::default();
        state.cpu.set16(WideRegister::BC, 0x0010)?;

        let op = Add16Operation(WideRegister::BC);

        assert_eq!(0x00, state.cpu.get(Register::H).unwrap());
        assert_eq!(0x00, state.cpu.get(Register::L).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0x00, state.cpu.get(Register::H).unwrap());
        assert_eq!(0x10, state.cpu.get(Register::L).unwrap());
        assert!(!state.cpu.has_flag(Flags::HALF_CARRY));
        assert!(!state.cpu.has_flag(Flags::CARRY));

        Ok(())
    }

    #[test]
    fn it_adds_the_upper_byte() -> crate::Result<()> {
        let mut state = State::default();
        state.cpu.set16(WideRegister::BC, 0x0F10)?;
        let op = Add16Operation(WideRegister::BC);

        assert_eq!(0x00, state.cpu.get(Register::H).unwrap());
        assert_eq!(0x00, state.cpu.get(Register::L).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0x0F, state.cpu.get(Register::H).unwrap());
        assert_eq!(0x10, state.cpu.get(Register::L).unwrap());
        assert!(!state.cpu.has_flag(Flags::HALF_CARRY));
        assert!(!state.cpu.has_flag(Flags::CARRY));

        Ok(())
    }

    #[test]
    fn it_sets_half_carry() -> crate::Result<()> {
        let mut state = State::default();
        state.cpu.set(Register::L, 0xFF)?;
        state.cpu.set(Register::C, 1)?;

        let op = Add16Operation(WideRegister::BC);

        assert_eq!(0x00, state.cpu.get(Register::H).unwrap());
        assert_eq!(0xFF, state.cpu.get(Register::L).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0x01, state.cpu.get(Register::H).unwrap());
        assert_eq!(0x00, state.cpu.get(Register::L).unwrap());
        assert!(state.cpu.has_flag(Flags::HALF_CARRY));
        assert!(!state.cpu.has_flag(Flags::CARRY));

        Ok(())
    }

    #[test]
    fn it_sets_carry() -> crate::Result<()> {
        let mut state = State::default();
        state.cpu.set16(WideRegister::HL, 0xFFFF)?;
        state.cpu.set16(WideRegister::BC, 0x0001)?;
        let op = Add16Operation(WideRegister::BC);

        state.cpu.set(Register::H, 0xFF).unwrap();
        state.cpu.set(Register::L, 0xFF).unwrap();

        assert_eq!(0xFF, state.cpu.get(Register::H).unwrap());
        assert_eq!(0xFF, state.cpu.get(Register::L).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0x00, state.cpu.get(Register::H).unwrap());
        assert_eq!(0x00, state.cpu.get(Register::L).unwrap());
        assert!(state.cpu.has_flag(Flags::HALF_CARRY));
        assert!(state.cpu.has_flag(Flags::CARRY));

        Ok(())
    }

    #[test]
    fn it_clears_the_subtraction_flag() {
        let mut state = State::default();
        state.cpu.set_flag(Flags::SUBTRACTION);

        Add16Operation(WideRegister::BC).act(&mut state).unwrap();

        assert_ne!(true, state.cpu.has_flag(Flags::SUBTRACTION));
    }
}
