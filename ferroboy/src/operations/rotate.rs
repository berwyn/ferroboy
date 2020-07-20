use crate::{
    assembly::{AssemblyInstruction, AssemblyInstructionBuilder, Disassemble},
    operations::Operation,
    state::State,
    system::{Cartridge, Flags, Register},
};

#[derive(Copy, Clone, Debug)]
pub struct RLCAOperation;

impl Operation for RLCAOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        let accumulator = state.cpu.get(Register::A);
        let tail = accumulator >> 7;

        let new_value = accumulator << 1 | tail;

        state.cpu.set(Register::A, new_value);
        state.cpu.increment_clock(4);

        state.cpu.clear_flag(Flags::ZERO);
        state.cpu.clear_flag(Flags::SUBTRACTION);
        state.cpu.clear_flag(Flags::HALF_CARRY);
        state.cpu.set_flag_value(Flags::CARRY, tail == 1);

        Ok(())
    }
}

impl Disassemble for RLCAOperation {
    fn disassemble(&self, _: &Cartridge, _: usize) -> crate::Result<AssemblyInstruction> {
        self.describe()
    }

    fn describe(&self) -> crate::Result<AssemblyInstruction> {
        AssemblyInstructionBuilder::new()
            .with_command("RLCA")
            .build()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RRCAOperation;

impl Operation for RRCAOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        let accumulator = state.cpu.get(Register::A);
        let head = accumulator & 0b0000_0001;

        let new_value = accumulator >> 1 | head << 7;

        state.cpu.set(Register::A, new_value);
        state.cpu.increment_clock(4);

        state.cpu.clear_flag(Flags::ZERO);
        state.cpu.clear_flag(Flags::SUBTRACTION);
        state.cpu.clear_flag(Flags::HALF_CARRY);
        state.cpu.set_flag_value(Flags::CARRY, head == 1);

        Ok(())
    }
}

impl Disassemble for RRCAOperation {
    fn disassemble(&self, _: &Cartridge, _: usize) -> crate::Result<AssemblyInstruction> {
        self.describe()
    }

    fn describe(&self) -> crate::Result<AssemblyInstruction> {
        AssemblyInstructionBuilder::new()
            .with_command("RRCA")
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod rlca {
        use super::*;

        mod operations {
            use super::*;

            #[test]
            fn it_rotates_the_accumulator() {
                let mut state = State::default();
                state.cpu.set(Register::A, 0b1111_1110);

                RLCAOperation.act(&mut state).unwrap();

                assert_eq!(0b1111_1101, state.cpu.get(Register::A));
                assert!(state.cpu.has_flag(Flags::CARRY));
            }

            #[test]
            fn it_clears_other_flags() {
                let mut state = State::default();
                state.cpu.set(Register::A, 0b1000_0000);
                state.cpu.set_flag(Flags::ZERO);
                state.cpu.set_flag(Flags::SUBTRACTION);
                state.cpu.set_flag(Flags::HALF_CARRY);

                RLCAOperation.act(&mut state).unwrap();

                assert!(!state.cpu.has_flag(Flags::ZERO));
                assert!(!state.cpu.has_flag(Flags::SUBTRACTION));
                assert!(!state.cpu.has_flag(Flags::HALF_CARRY));
                assert!(state.cpu.has_flag(Flags::CARRY));
            }
        }

        mod disassemble {
            use super::*;

            #[test]
            fn it_disassembles_correctly() {
                assert_eq!(
                    "RLCA",
                    RLCAOperation
                        .disassemble(&Cartridge::default(), 0)
                        .unwrap()
                        .to_string()
                )
            }
        }
    }

    mod rrca {
        use super::*;

        mod operations {
            use super::*;

            #[test]
            fn it_rotates_the_accumulator() {
                let mut state = State::default();
                state.cpu.set(Register::A, 0b1111_1101);

                RRCAOperation.act(&mut state).unwrap();

                assert_eq!(0b1111_1110, state.cpu.get(Register::A));
                assert!(state.cpu.has_flag(Flags::CARRY));
            }

            #[test]
            fn it_clears_other_flags() {
                let mut state = State::default();
                state.cpu.set(Register::A, 0b0000_0001);
                state.cpu.set_flag(Flags::ZERO);
                state.cpu.set_flag(Flags::SUBTRACTION);
                state.cpu.set_flag(Flags::HALF_CARRY);

                RRCAOperation.act(&mut state).unwrap();

                assert!(!state.cpu.has_flag(Flags::ZERO));
                assert!(!state.cpu.has_flag(Flags::SUBTRACTION));
                assert!(!state.cpu.has_flag(Flags::HALF_CARRY));
                assert!(state.cpu.has_flag(Flags::CARRY));
            }
        }

        mod disassemble {
            use super::*;

            #[test]
            fn it_disassembles_correctly() {
                assert_eq!(
                    "RRCA",
                    RRCAOperation
                        .disassemble(&Cartridge::default(), 0)
                        .unwrap()
                        .to_string()
                )
            }
        }
    }
}
