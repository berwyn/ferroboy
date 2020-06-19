use crate::{
    assembly::{AssemblyInstruction, AssemblyInstructionBuilder, Disassemble},
    operations::Operation,
    system::{Flags, Register},
};

/// Decrements a register.
///
/// # Opcode Reference
/// ## Assembly definition
/// ```a
/// DEC A
/// ```
///
/// ## Runtime
/// | Metric | Size |
/// |:-------|:-----|
/// | Length | 1    |
/// | Cycles | 4    |
///
/// ## Flags
/// | Flag        | Value        |
/// |:------------|:-------------|
/// | Zero        | Set          |
/// | Subtraction | 1            |
/// | Half-Carry  | Set          |
/// | Carry       | Not Affected |
///
/// # Examples
/// ```rs
/// Dec8Operation(Register::B).act(&mut state).unwrap();
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Dec8Operation(Register);

impl Operation for Dec8Operation {
    fn act(&self, state: &mut crate::State) -> crate::Result<()> {
        let current_value = state.cpu.get(self.0);
        let new_value = current_value.wrapping_sub(1);

        state.cpu.set(self.0, new_value);
        state.cpu.increment_clock(4);

        state.cpu.set_flag_value(Flags::ZERO, new_value == 0);
        state.cpu.set_flag(Flags::SUBTRACTION);
        // TODO(berwyn): How does half-carry work with DEC?

        Ok(())
    }
}
impl Disassemble for Dec8Operation {
    fn disassemble(&self, _: &crate::Cartridge, _: usize) -> crate::Result<AssemblyInstruction> {
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
        use crate::State;

        #[test]
        fn it_decrements_a_register() {
            let mut state = State::default();
            state.cpu.set(Register::B, 1);

            Dec8Operation(Register::B).act(&mut state).unwrap();

            assert_eq!(0, state.cpu.get(Register::B));
        }

        #[test]
        fn it_should_set_the_zero_flag() {
            let mut state = State::default();
            state.cpu.set(Register::B, 1);

            Dec8Operation(Register::B).act(&mut state).unwrap();

            assert!(state.cpu.has_flag(Flags::ZERO));

            Dec8Operation(Register::B).act(&mut state).unwrap();

            assert!(!state.cpu.has_flag(Flags::ZERO));
        }

        #[test]
        fn it_should_set_the_subtraction_flag() {
            let mut state = State::default();

            Dec8Operation(Register::B).act(&mut state).unwrap();

            assert!(state.cpu.has_flag(Flags::SUBTRACTION));
        }

        #[test]
        #[should_panic]
        fn it_should_set_the_half_carry_flag() {
            todo!()
        }

        #[test]
        fn it_should_not_modify_the_carry_flag() {
            let mut state = State::default();

            Dec8Operation(Register::B).act(&mut state).unwrap();

            assert!(!state.cpu.has_flag(Flags::CARRY));

            state.cpu.set_flag(Flags::CARRY);

            Dec8Operation(Register::B).act(&mut state).unwrap();

            assert!(state.cpu.has_flag(Flags::CARRY));
        }
    }

    mod disassemble {
        use super::*;
        use crate::Cartridge;

        #[test]
        fn it_disassembles_properly() {
            assert_eq!(
                "DEC B",
                Dec8Operation(Register::B)
                    .disassemble(&Cartridge::default(), 0)
                    .unwrap()
                    .to_string()
            )
        }
    }
}
