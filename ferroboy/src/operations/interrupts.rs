use crate::{
    assembly::{AssemblyInstruction, AssemblyInstructionBuilder, Disassemble},
    operations::Operation,
    Cartridge, State,
};

#[derive(Clone, Copy, Debug)]
pub struct DisableInterruptsOperation;

impl Operation for DisableInterruptsOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        state.cpu.disable_interrupts();
        state.cpu.increment_clock(4);

        Ok(())
    }
}

impl Disassemble for DisableInterruptsOperation {
    fn disassemble(&self, _: &Cartridge, _: usize) -> crate::Result<AssemblyInstruction> {
        self.describe()
    }

    fn describe(&self) -> crate::Result<AssemblyInstruction> {
        AssemblyInstructionBuilder::new().with_command("DI").build()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct EnableInterruptsOperation;

impl Operation for EnableInterruptsOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        state.cpu.enable_interrupts();
        state.cpu.increment_clock(4);

        Ok(())
    }
}

impl Disassemble for EnableInterruptsOperation {
    fn disassemble(&self, _: &Cartridge, _: usize) -> crate::Result<AssemblyInstruction> {
        self.describe()
    }

    fn describe(&self) -> crate::Result<AssemblyInstruction> {
        AssemblyInstructionBuilder::new().with_command("EI").build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod disable {
        use super::*;

        #[test]
        fn it_disables_interrupts() {
            let mut state = State::default();
            assert!(state.cpu.interrupts_enabled());

            DisableInterruptsOperation.act(&mut state).unwrap();

            assert!(!state.cpu.interrupts_enabled());
        }

        #[test]
        fn it_disassembles_correctly() {
            let instruction = DisableInterruptsOperation
                .disassemble(&Cartridge::default(), 0)
                .unwrap();

            assert_eq!("DI", instruction.to_string());
        }
    }

    mod enable {
        use super::*;

        #[test]
        fn it_enables_interrupts() {
            let mut state = State::default();
            state.cpu.disable_interrupts();

            assert!(!state.cpu.interrupts_enabled());

            EnableInterruptsOperation.act(&mut state).unwrap();

            assert!(state.cpu.interrupts_enabled());
        }

        #[test]
        fn it_disassembles_correctly() {
            let instruction = EnableInterruptsOperation
                .disassemble(&Cartridge::default(), 0)
                .unwrap();

            assert_eq!("EI", instruction.to_string());
        }
    }
}
