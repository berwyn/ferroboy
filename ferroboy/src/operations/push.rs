use core::convert::TryInto;

use crate::{
    assembly::{AssemblyInstruction, AssemblyInstructionBuilder, Disassemble},
    error::OperationError,
    operations::Operation,
    state::State,
    system::{Cartridge, WideRegister},
};

/// Pushes a value from a wide register to the location of the stack pointer.
///
/// # Opcode Reference
/// ## Assembly defintion
/// ```a
/// PUSH BC
/// ```
///
/// ## Runtime
/// | Metric | Size |
/// |:-------|:-----|
/// | Length | 1    |
/// | Cycles | 16   |
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
/// PushOperation(WideRegister::BC).act(&mut state).unwrap();
/// ```
///
/// # Errors
/// - `PC` and `SP` are not valid register to PUSH from
#[derive(Copy, Clone, Debug)]
pub struct PushOperation(pub WideRegister);

impl Operation for PushOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        match self.0 {
            WideRegister::SP | WideRegister::PC => {
                Err(OperationError::InvalidWideRegister(self.0).into())
            }
            _ => {
                let (high, low) = self.0.try_into().unwrap();
                let (high, low) = (state.cpu.get(high), state.cpu.get(low));

                let address = state.cpu.get16(WideRegister::SP);
                state.mmu[address - 1] = high;
                state.mmu[address - 2] = low;

                state.cpu.set16(WideRegister::SP, address - 2);
                state.cpu.increment_clock(16);

                Ok(())
            }
        }
    }
}

impl Disassemble for PushOperation {
    fn disassemble(&self, _: &Cartridge, _: usize) -> crate::Result<AssemblyInstruction> {
        self.describe()
    }

    fn describe(&self) -> crate::Result<AssemblyInstruction> {
        AssemblyInstructionBuilder::new()
            .with_command("PUSH")
            .with_arg(self.0)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod operation {
        use super::*;

        #[test]
        fn it_pushes_a_register_to_the_stack_pointer() {
            let mut state = State::default();
            state.cpu.set16(WideRegister::HL, 0xBEEF);
            state.cpu.set16(WideRegister::SP, 0x02);

            PushOperation(WideRegister::HL).act(&mut state).unwrap();

            assert_eq!(0x00, state.cpu.get16(WideRegister::SP));
            assert_eq!(0xEF, state.mmu[0x00]);
            assert_eq!(0xBE, state.mmu[0x01]);
        }

        #[test]
        #[should_panic(expected = "Cannot PUSH with PC or SP")]
        fn it_disallows_sp() {
            PushOperation(WideRegister::SP)
                .act(&mut State::default())
                .unwrap();
        }

        #[test]
        #[should_panic(expected = "Cannot PUSH with PC or SP")]
        fn it_disallows_pc() {
            PushOperation(WideRegister::PC)
                .act(&mut State::default())
                .unwrap();
        }
    }

    mod disassemble {
        use super::*;

        #[test]
        fn it_disassembles_correctly() {
            assert_eq!(
                "PUSH HL",
                PushOperation(WideRegister::HL)
                    .disassemble(&Cartridge::default(), 0)
                    .unwrap()
                    .to_string()
            )
        }
    }
}
