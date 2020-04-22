use crate::assembly::{AssemblyInstruction, AssemblyInstructionBuilder};
use crate::operations::Operation;
use crate::state::State;
use crate::system::{Flags, Register};

/// Add the contents of one register to another
///
/// # Opcode Reference
/// ## Assembly definition
/// ```a
/// ADD A,B
/// ```
/// ## Runtime
/// | Metric | Size |
/// |:-------|:-----|
/// | Length | 1 |
/// | Cycles | 4 |
///
/// ## Flags
/// | Flag | Value |
/// |:-----|:------|
/// | Zero | Set |
/// | Subtraction | 0 |
/// | Half-Carry | Set |
/// | Carry | Set |
///
/// # Examples
/// ```rs
/// let operation = Add8Operation(Register::A, Register::B);
/// operation.act(&mut state).unwrap();
/// ```
///
/// # Errors
/// - The operation may fail if a 16-bit register is provided for either argument.
#[derive(Debug)]
pub struct Add8Operation(pub Register, pub Register);

impl Operation for Add8Operation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        let value = state.cpu.get(self.1)?;

        state.cpu.clear_flag(Flags::SUBTRACTION);

        // FIXME: This probably shouldn't ever fail, revisit this
        state.cpu.mutate(self.0, |old| old + value)?;

        // TODO: H + C

        state.cpu.increment_clock(4);

        Ok(())
    }
}

impl core::convert::TryFrom<Add8Operation> for AssemblyInstruction {
    type Error = String;

    fn try_from(value: Add8Operation) -> Result<AssemblyInstruction, Self::Error> {
        let instruction = AssemblyInstructionBuilder::new()
            .with_command("ADD")
            .with_arg(value.0)
            .with_arg(value.1)
            .build()?;

        Ok(instruction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_disassembles_correctly() {
        use core::convert::TryInto;

        let add = Add8Operation(Register::A, Register::B);
        let add_instruction: AssemblyInstruction = add.try_into().unwrap();

        assert_eq!("ADD A,B", add_instruction.to_string());
    }

    #[test]
    fn it_adds_two_registers() -> crate::Result<()> {
        let mut state = State::default();
        state.cpu.set(Register::A, 0x10)?;
        state.cpu.set(Register::B, 0x06)?;

        Add8Operation(Register::A, Register::B).act(&mut state)
    }

    #[test]
    fn it_clears_the_zero_flag() {
        use crate::system::Flags;

        let mut state = State::default();
        state.cpu.set_flag(Flags::SUBTRACTION);

        Add8Operation(Register::A, Register::B)
            .act(&mut state)
            .unwrap();

        assert_eq!(false, state.cpu.has_flag(Flags::SUBTRACTION))
    }

    #[test]
    #[should_panic]
    fn it_sets_the_half_carry_flag() {
        todo!();
    }

    #[test]
    #[should_panic]
    fn it_sets_the_carry_flag() {
        todo!();
    }

    #[test]
    #[should_panic]
    fn it_sets_the_zero_flag() {
        todo!();
    }

    #[test]
    fn it_returns_an_error_for_16bit_registers() {
        let mut state = State::default();
        state.cpu.set(Register::A, 0x10).unwrap();
        state.cpu.set(Register::B, 0x06).unwrap();

        assert!(
            Add8Operation(Register::AF, Register::B)
                .act(&mut state)
                .is_err(),
            "Add8Operation shouldn't accept left-hand 16-bit registers"
        );

        assert!(
            Add8Operation(Register::B, Register::AF)
                .act(&mut state)
                .is_err(),
            "Add8Operation shouldn't accept right-hand 16-bit registers"
        );
    }
}
