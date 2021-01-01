use crate::{
    assembly::{AssemblyInstruction, AssemblyInstructionBuilder, Disassemble},
    helpers::format_hex_address,
    operations::Operation,
    system::WideRegister,
    Cartridge, State,
};

/// Pushes the current program counter onto the stack and then jumps to a pre-set address.
///
/// # Opcode reference
/// ## Assembly definition
/// ```a
/// RST 00H
/// ```
///
/// ## Runtime
/// | Metric | Size |
/// |:-------|:-----|
/// | Length | 1    |
/// | Cycles | 16   |
///
/// ## Flags
/// | Flag        | Value     |
/// |:------------|:----------|
/// | Zero        | Unchanged |
/// | Subtraction | Unchanged |
/// | Half-Carry  | Unchanged |
/// | Carry       | Unchanged |
///
/// # Examples
/// ```rs
/// RstOperation(0x38).act(&mut state).unwrap();
/// ```
#[derive(Debug)]
pub struct RstOperation(pub u16);
// Note we allow any address, but the opcode set has a fixed few address

impl Operation for RstOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        let address = state.cpu.get16(WideRegister::PC);
        let target = state.cpu.get16(WideRegister::SP) - 2;

        state.mmu[target] = (address >> 8) as u8;
        state.mmu[target + 1] = address as u8;

        state.cpu.set16(WideRegister::PC, self.0);

        state.cpu.increment_clock(16);

        Ok(())
    }
}

impl Disassemble for RstOperation {
    fn disassemble(
        &self,
        _cartridge: &Cartridge,
        _offset: usize,
    ) -> crate::Result<AssemblyInstruction> {
        self.describe()
    }

    fn describe(&self) -> crate::Result<AssemblyInstruction> {
        AssemblyInstructionBuilder::new()
            .with_command("RST")
            .with_arg(format_hex_address(self.0, false))
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod operation {
        use pretty_assertions::assert_eq;

        use crate::helpers::word_to_u16;

        use super::*;

        fn setup_state() -> State {
            let mut state = State::default();
            state.cpu.set16(WideRegister::PC, 0xDEAD);
            state.cpu.set16(WideRegister::SP, 0xBEF1);
            state
        }

        #[test]
        fn it_pushes_the_program_counter() {
            let mut state = setup_state();

            RstOperation(0x38).act(&mut state).unwrap();

            let (high, low) = (state.mmu[0xBEEF], state.mmu[0xBEF0]);
            let address = word_to_u16((high, low));

            assert_eq!(0xDEAD, address);
        }

        #[test]
        fn it_jumps_to_the_vector() {
            let mut state = setup_state();

            RstOperation(0x38).act(&mut state).unwrap();

            let program_counter = state.cpu.get16(WideRegister::PC);

            assert_eq!(0x38, program_counter);
        }
    }

    mod disassemble {
        use pretty_assertions::assert_eq;

        use super::*;

        #[test]
        fn it_describes_properly() {
            let instruction = RstOperation(0x38).describe().unwrap();

            assert_eq!("RST $38", instruction.to_string());
        }

        #[test]
        fn it_disassembles_properly() {
            let instruction = RstOperation(0x38)
                .disassemble(&Cartridge::default(), 0)
                .unwrap();

            assert_eq!("RST $38", instruction.to_string())
        }
    }
}
