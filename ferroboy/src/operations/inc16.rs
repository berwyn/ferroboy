use core::convert::TryInto;

use crate::assembly::{AssemblyInstruction, AssemblyInstructionBuilder, Disassemble};
use crate::operations::Operation;
use crate::state::State;
use crate::system::{Flags, WideRegister};

/// Increments a singular register.
///
/// # Opcode Reference
/// ## Assembly Definition
/// ```a
/// INC BC
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
/// | Subtraction | Not Affected |
/// | Half-Carry | Not Affected |
/// | Carry | Not Affected |
///
/// # Examples
/// ```rs
/// let operation = Inc16Operation(Register::BC);
/// operation.act(&mut state).unwrap();
/// ```
///
/// # Errors
/// - The operation may fail if an 8-bit register is provided.
#[derive(Clone, Copy, Debug)]
pub struct Inc16Operation(pub WideRegister);

impl Operation for Inc16Operation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        match self.0 {
            WideRegister::PC | WideRegister::SP => {
                state.cpu.set16(self.0, state.cpu.get16(self.0)? + 1)?;
            }
            _ => {
                let (high, low) = self.0.try_into()?;

                let mut lower = u16::from(state.cpu.get(low)?);
                lower += 1;
                state.cpu.set(low, lower as u8)?;

                if lower / 0xFF > 0 {
                    state.cpu.set_flag(Flags::HALF_CARRY);
                    let mut upper = u16::from(state.cpu.get(high)?);
                    upper += 1;

                    if upper / 0xFF > 0 {
                        state.cpu.set_flag(Flags::CARRY);
                    }

                    state.cpu.set(high, upper as u8)?;
                }
            }
        };

        state.cpu.increment_clock(8);

        Ok(())
    }
}

impl Disassemble for Inc16Operation {
    fn disassemble(&self, _: &mut State) -> crate::Result<AssemblyInstruction> {
        AssemblyInstructionBuilder::new()
            .with_command("INC")
            .with_arg(self.0)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::system::Register;

    #[test]
    fn it_disassembles_correctly() {
        let inc = Inc16Operation(WideRegister::BC);
        let inc_instruction = inc.disassemble(&mut State::default()).unwrap();

        assert_eq!("INC BC", inc_instruction.to_string());
    }

    #[test]
    fn it_increments_the_lower_byte() {
        let mut state = State::default();
        let op = Inc16Operation(WideRegister::BC);

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());
        assert_eq!(0x00, state.cpu.get(Register::C).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());
        assert_eq!(0x01, state.cpu.get(Register::C).unwrap());
    }

    #[test]
    fn it_increments_the_upper_byte() {
        let mut state = State::default();
        let op = Inc16Operation(WideRegister::BC);

        state.cpu.set(Register::C, 0xFF).unwrap();

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());
        assert_eq!(0xFF, state.cpu.get(Register::C).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0x01, state.cpu.get(Register::B).unwrap());
        assert_eq!(0x00, state.cpu.get(Register::C).unwrap());
        assert!(state.cpu.has_flag(Flags::HALF_CARRY));
    }

    #[test]
    fn it_wraps_over() {
        let mut state = State::default();
        let op = Inc16Operation(WideRegister::BC);

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
}
