use crate::assembly::{AssemblyInstruction, AssemblyInstructionBuilder, Disassemble};
use crate::operations::Operation;
use crate::state::State;
use crate::system::{Cartridge, Register};

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
#[derive(Clone, Copy, Debug)]
pub struct Inc8Operation(pub Register);

impl Operation for Inc8Operation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        let mut temp = u16::from(state.cpu.get(self.0));
        temp += 1;

        // FIXME: Set flags as needed
        state.cpu.set(self.0, temp as u8);
        state.cpu.increment_clock(4);

        Ok(())
    }
}

impl Disassemble for Inc8Operation {
    fn disassemble(&self, _: &Cartridge, _: usize) -> crate::Result<AssemblyInstruction> {
        AssemblyInstructionBuilder::new()
            .with_command("INC")
            .with_arg(self.0)
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

        assert_eq!(1, state.cpu.get(Register::A));
    }

    #[test]
    fn it_disassembles_correctly() {
        let instruction = Inc8Operation(Register::A)
            .disassemble(&Cartridge::default(), 0)
            .unwrap();

        assert_eq!("INC A", instruction.to_string());
    }
}
