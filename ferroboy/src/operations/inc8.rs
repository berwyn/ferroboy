use crate::assembly::{AssemblyInstruction, AssemblyInstructionBuilder};
use crate::operations::Operation;
use crate::state::State;
use crate::system::Register;

/// Increments a single register.
///
/// # Opcode Reference
/// ## Assembly definition
/// ```a
/// INC A
/// ```
///
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
/// | Carry | Not Affected |
///
/// # Examples
/// ```rs
/// let operation = Inc8Operation(Register::A);
/// operation.act(&mut state).unwrap();
/// ```
///
/// # Errors
/// - The operation may fail if a 16-bit register is provided.
#[derive(Debug)]
pub struct Inc8Operation(pub Register);

impl Operation for Inc8Operation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        let mut temp = u16::from(state.cpu.get(self.0)?);
        temp += 1;

        // FIXME: Set flags as needed
        state.cpu.set(self.0, temp as u8)?;
        state.cpu.increment_clock(4);

        Ok(())
    }
}

impl core::convert::TryFrom<Inc8Operation> for AssemblyInstruction {
    type Error = String;

    fn try_from(value: Inc8Operation) -> Result<AssemblyInstruction, Self::Error> {
        AssemblyInstructionBuilder::new()
            .with_command("INC")
            .with_arg(value.0)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_increments_the_register() {
        let mut state = State::default();

        Inc8Operation(Register::A).act(&mut state).unwrap();

        assert_eq!(1, state.cpu.get(Register::A).unwrap());
    }

    #[test]
    fn it_disassembles_correctly() {
        use core::convert::TryInto;
        let assembly_instruction: AssemblyInstruction =
            Inc8Operation(Register::A).try_into().unwrap();

        assert_eq!("INC A", assembly_instruction.to_string());
    }
}
