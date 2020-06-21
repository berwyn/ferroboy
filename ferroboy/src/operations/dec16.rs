use crate::{
    assembly::{AssemblyInstruction, AssemblyInstructionBuilder, Disassemble},
    operations::Operation,
    state::State,
    system::{Cartridge, WideRegister, ALU},
};

/// Decrements a wide register.
///
/// # Opcode Reference
/// ## Assembly definition
/// ```a
/// DEC BC
/// ```
///
/// ## Runtime
/// | Metric | Size |
/// |:-------|:-----|
/// | Length | 1    |
/// | Cycles | 8    |
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
/// Dec16Operation(WideRegister::BC).act(&mut state).unwrap();
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Dec16Operation(pub WideRegister);

impl Operation for Dec16Operation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        let current_value = state.cpu.get16(self.0);
        let (new_value, _, _) = current_value.alu_sub(1);

        state.cpu.set16(self.0, new_value);
        state.cpu.increment_clock(8);

        Ok(())
    }
}

impl Disassemble for Dec16Operation {
    fn disassemble(&self, _: &Cartridge, _: usize) -> crate::Result<AssemblyInstruction> {
        AssemblyInstructionBuilder::new()
            .with_command("DEC")
            .with_arg(self.0)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod operation {
        use super::*;
        use crate::system::{Flags, Register};

        #[test]
        fn it_should_decrement_a_register() {
            let mut state = State::default();
            state.cpu.set16(WideRegister::BC, 0x0100);

            Dec16Operation(WideRegister::BC).act(&mut state).unwrap();

            assert_eq!(0x00, state.cpu.get(Register::B));
            assert_eq!(0xFF, state.cpu.get(Register::C));
        }

        #[test]
        fn it_doesnt_affect_flags() {
            let mut state = State::default();
            state.cpu.set_flag(Flags::ZERO);
            state.cpu.set_flag(Flags::SUBTRACTION);
            state.cpu.set_flag(Flags::HALF_CARRY);
            state.cpu.set_flag(Flags::CARRY);

            Dec16Operation(WideRegister::BC).act(&mut state).unwrap();

            assert!(state.cpu.has_flag(Flags::ZERO));
            assert!(state.cpu.has_flag(Flags::SUBTRACTION));
            assert!(state.cpu.has_flag(Flags::HALF_CARRY));
            assert!(state.cpu.has_flag(Flags::CARRY));
        }
    }

    mod disassemble {
        use super::*;

        #[test]
        fn it_disassembles_correctly() {
            assert_eq!(
                "DEC BC",
                Dec16Operation(WideRegister::BC)
                    .disassemble(&Cartridge::default(), 0)
                    .unwrap()
                    .to_string()
            );
        }
    }
}
