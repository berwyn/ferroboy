use crate::{
    assembly::{AssemblyInstruction, AssemblyInstructionBuilder, Disassemble},
    operations::Operation,
    State,
};

/// Sets the CPU into the CB prefix mode.
///
/// # Opcode reference
/// ## Assembly definition
/// ```a
/// PREFIX CB
/// ```
///
/// ## Runtime
/// | Metric | Size |
/// |:-------|:-----|
/// | Length | 1    |
/// | Cycles | 4    |
///
/// ## Flags
/// | Flag        | Value     |
/// |:------------|:----------|
/// | Zero        | Unchanged |
/// | Subtraction | Unchanged |
/// | Half-Carry  | Unchanged |
/// | Carry       | Unchanged |
///
/// # Example
/// ```rs
/// PrefixOperation.act(&mut state)?;
/// ```
#[derive(Copy, Clone, Debug)]
pub struct PrefixOperation;

impl Operation for PrefixOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        state.cpu.in_prefix_mode = !state.cpu.in_prefix_mode;
        Ok(())
    }
}

impl Disassemble for PrefixOperation {
    fn disassemble(&self, _: &crate::Cartridge, _: usize) -> crate::Result<AssemblyInstruction> {
        self.describe()
    }

    fn describe(&self) -> crate::Result<AssemblyInstruction> {
        AssemblyInstructionBuilder::new()
            .with_command("PREFIX CB")
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod operation {
        use super::*;

        #[test]
        fn it_sets_the_prefix_flag() {
            let mut state = State::default();

            assert!(!state.cpu.in_prefix_mode);

            PrefixOperation.act(&mut state).unwrap();

            assert!(state.cpu.in_prefix_mode);
        }
    }

    mod disassemble {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_describes_property() {
            let instruction = PrefixOperation.describe().unwrap();

            assert_eq!("PREFIX CB", instruction.to_string());
        }

        #[test]
        fn it_disassembles_correctly() {
            let instruction = PrefixOperation
                .disassemble(&crate::Cartridge::default(), 0)
                .unwrap();

            assert_eq!("PREFIX CB", instruction.to_string());
        }
    }
}
