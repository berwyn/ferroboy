use crate::{
    assembly::{AssemblyInstruction, AssemblyInstructionBuilder, Disassemble},
    operations::Operation,
    state::State,
    system::Cartridge,
};

/// Causes the processor and screen to stop until an interrupt occurs.
///
/// # Opcode Reference
/// ## Assembly definition
/// ```a
/// HALT
/// ```
///
/// ## Runtime
/// | Metric | Size |
/// |:-------|:-----|
/// | Length | 1    |
/// | Cycles | 4    |
///
/// ## Flags
/// | Flag        | Value      |
/// |:------------|:-----------|
/// | Zero        | Unaffected |
/// | Subtraction | Unaffected |
/// | Half-Carry  | Unaffected |
/// | Carry       | Unaffected |
///
/// # Examples
/// ```rs
/// HaltOperation.act(&mut state).unwrap();
/// ```
///
/// # Errors
/// This operation cannot fail
#[derive(Copy, Clone, Debug)]
pub struct HaltOperation;

impl Operation for HaltOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        state.cpu.halt();
        state.cpu.increment_clock(4);

        Ok(())
    }
}
impl Disassemble for HaltOperation {
    fn disassemble(&self, _: &Cartridge, _: usize) -> crate::Result<AssemblyInstruction> {
        self.describe()
    }

    fn describe(&self) -> crate::Result<AssemblyInstruction> {
        AssemblyInstructionBuilder::new()
            .with_command("HALT")
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod operation {
        use super::*;

        #[test]
        fn it_halts_the_cpu() {
            let mut state = State::default();

            HaltOperation.act(&mut state).unwrap();

            assert!(state.cpu.is_halted());
            assert!(state.cpu.interrupts_enabled());
        }
    }

    mod disassemble {
        use super::*;

        #[test]
        fn it_disassembles_correctly() {
            assert_eq!(
                "HALT",
                HaltOperation
                    .disassemble(&Cartridge::default(), 0)
                    .unwrap()
                    .to_string()
            );
        }
    }
}
