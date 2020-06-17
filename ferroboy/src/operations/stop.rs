use crate::assembly::{AssemblyInstruction, AssemblyInstructionBuilder, Disassemble};
use crate::operations::Operation;
use crate::state::State;

/// Causes the processor and screen to stop until a button is pressed.
///
/// # Opcode Reference
/// ## Assembly definition
/// ```a
/// STOP 0
/// ```
///
/// ## Runtime
/// | Metric | Size |
/// |:-------|:-----|
/// | Length | 2    |
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
/// StopOperation.act(&mut state).unwrap();
/// ```
///
/// # Errors
/// This operation cannot fail
#[derive(Clone, Copy, Debug)]
pub struct StopOperation;

impl Operation for StopOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        state.cpu.disable_interrupts();
        state.cpu.halt();
        state.cpu.increment_clock(4);

        Ok(())
    }
}

impl Disassemble for StopOperation {
    fn disassemble(&self, _: &mut State) -> crate::Result<AssemblyInstruction> {
        AssemblyInstructionBuilder::new()
            .with_command("STOP")
            .with_arg("0")
            .with_size(2)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod operation {
        use super::*;

        #[test]
        fn it_stops_the_cpu() {
            let mut state = State::default();

            StopOperation.act(&mut state).unwrap();

            assert!(state.cpu.is_halted());
            assert!(!state.cpu.interrupts_enabled());
        }
    }

    mod disassemble {
        use super::*;

        #[test]
        fn it_disassembles_properly() {
            assert_eq!(
                "STOP 0",
                StopOperation
                    .disassemble(&mut State::default())
                    .unwrap()
                    .to_string()
            );
        }
    }
}
