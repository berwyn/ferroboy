use crate::{
    assembly::{AssemblyInstruction, AssemblyInstructionBuilder, Disassemble},
    operations::Operation,
    system::Cartridge,
    State,
};

/// A non-operation.
///
/// # Opcode Reference
/// ## Assembly definition
/// ```a
/// NOP
/// ```
///
/// ## Runtime
/// | Metric | Size |
/// |:-------|:-----|
/// | Length | 1    |
/// | Cycles | 4    |
///
/// ## Flags
/// | Flag          | Value        |
/// |:--------------|:-------------|
/// | Zero          | Not Affected |
/// | Subtraction   | Not Affected |
/// | Half-Carry    | Not Affected |
/// | Carry         | Not Affected |
#[derive(Clone, Copy, Debug)]
pub struct NopOperation;

impl Operation for NopOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        state.cpu.increment_clock(4);
        Ok(())
    }
}

impl Disassemble for NopOperation {
    fn disassemble(&self, _: &Cartridge, _: usize) -> crate::Result<AssemblyInstruction> {
        self.describe()
    }

    fn describe(&self) -> crate::Result<AssemblyInstruction> {
        AssemblyInstructionBuilder::new()
            .with_command("NOP")
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_disassembles_correctly() {
        let op = NopOperation;
        let instruction = op.disassemble(&Cartridge::default(), 0).unwrap();

        assert_eq!("NOP", instruction.to_string());
    }
}
