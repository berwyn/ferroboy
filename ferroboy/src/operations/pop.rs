use core::convert::TryInto;

use crate::{
    assembly::{AssemblyInstruction, AssemblyInstructionBuilder, Disassemble},
    error::OperationError,
    operations::Operation,
    state::State,
    system::{Cartridge, WideRegister},
};

/// Pops the value at the Stack Pointer into a wide register.
///
/// # Opcode Reference
/// ## Assembly definition
/// ```a
/// POP BC
/// ```
///
/// ## Runtime
/// | Metric | Size |
/// |:-------|:-----|
/// | Length | 1    |
/// | Cycles | 12   |
///
/// ## Flags
/// | Flag        | Value       |
/// |:------------|:------------|
/// | Zero        | Unaffected* |
/// | Subtraction | Unaffected* |
/// | Half-Carry  | Unaffected* |
/// | Carry       | Unaffected* |
///
/// \* When using `POP AF`, the flags will be set
///
/// # Examples
/// ```rs
/// PushOperation(WideRegister::BC).act(&mut state).unwrap()
/// ```
///
/// # Errors
/// - `PC` and `SP` are not valid registers to POP onto
#[derive(Copy, Clone, Debug)]
pub struct PopOperation(pub WideRegister);

impl Operation for PopOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        match self.0 {
            WideRegister::Pc | WideRegister::Sp => {
                Err(OperationError::InvalidWideRegister(self.0).into())
            }
            _ => {
                let (high, low) = self.0.try_into().unwrap();
                let address = state.cpu.get16(WideRegister::Sp);

                state.cpu.set(low, state.mmu[address]);
                state.cpu.set(high, state.mmu[address + 1]);
                state.cpu.set16(WideRegister::Sp, address + 2);
                state.cpu.increment_clock(12);

                Ok(())
            }
        }
    }
}

impl Disassemble for PopOperation {
    fn disassemble(&self, _: &Cartridge, _: usize) -> crate::Result<AssemblyInstruction> {
        self.describe()
    }

    fn describe(&self) -> crate::Result<AssemblyInstruction> {
        AssemblyInstructionBuilder::new()
            .with_command("POP")
            .with_arg(self.0)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod operation {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_copies_the_stack_pointer_to_a_register() {
            let mut state = State::default();
            state.mmu[0x00] = 0xEF;
            state.mmu[0x01] = 0xBE;

            PopOperation(WideRegister::Hl).act(&mut state).unwrap();

            assert_eq!(0xBEEF, state.cpu.get16(WideRegister::Hl));
            assert_eq!(0x02, state.cpu.get16(WideRegister::Sp));
        }

        // TODO(berwyn): Check the message
        #[test]
        #[should_panic]
        fn it_disallows_sp() {
            PopOperation(WideRegister::Sp)
                .act(&mut State::default())
                .unwrap();
        }

        // TODO(berwyn): Check the message
        #[test]
        #[should_panic]
        fn it_disallows_pc() {
            PopOperation(WideRegister::Pc)
                .act(&mut State::default())
                .unwrap();
        }
    }

    mod disassemble {
        use super::*;

        #[test]
        fn it_disassembles_correctly() {
            assert_eq!(
                "POP HL",
                PopOperation(WideRegister::Hl)
                    .disassemble(&Cartridge::default(), 0)
                    .unwrap()
                    .to_string()
            )
        }
    }
}
