use crate::{
    operations::Operation, AssemblyInstruction, AssemblyInstructionBuilder, Disassemble, State,
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
    fn disassemble(&self, _: &mut State) -> crate::Result<AssemblyInstruction> {
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
    fn disassemble(&self, _: &mut State) -> crate::Result<AssemblyInstruction> {
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
                .disassemble(&mut State::default())
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
                .disassemble(&mut State::default())
                .unwrap();

            assert_eq!("EI", instruction.to_string());
        }
    }
}
