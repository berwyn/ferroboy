use crate::assembly::{AssemblyInstruction, AssemblyInstructionBuilder};
use crate::operations::Operation;
use crate::state::State;
use crate::system::{Flags, Register};

#[derive(Debug)]
pub struct Add8Operation(pub Register, pub Register);

impl Operation for Add8Operation {
    fn act(&self, state: &mut State) -> Result<(), String> {
        let value = state.cpu.get(self.1)?;

        state.cpu.clear_flag(Flags::SUBTRACTION);
        state.cpu.mutate(self.0, |old| old + value)?;

        // TODO: H + C

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
