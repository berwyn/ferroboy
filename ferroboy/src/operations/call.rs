use crate::{
    assembly::AssemblyInstruction, assembly::AssemblyInstructionBuilder, assembly::Disassemble,
    helpers::word_to_u16, operations::Operation, system::Flags, system::WideRegister, Cartridge,
    State,
};

/// The condition under which the call should or should not be executed.
#[derive(Copy, Clone, Debug)]
pub enum CallCondition {
    Zero,
    NotZero,
    Carry,
    NotCarry,
}

impl std::fmt::Display for CallCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Zero => "Z",
                Self::NotZero => "NZ",
                Self::Carry => "C",
                Self::NotCarry => "NC",
            }
        )
    }
}

/// Jumps to the target register, preserving the program counter in the stack pointer.
///
/// # Opcode reference
/// ## Assembly definition
/// ```a
/// CALL $BEEF
/// ```
///
/// ## Runtime
/// | Metric | Size |
/// |:-------|:-----|
/// | Length | 3    |
/// | Cycles | 24\* |
///
/// \* In cases where a conditional is provided and not satisfied, the call is 12 cycles.
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
/// CallOperation(0xBEEF, None).act(&mut state).unwrap()
/// ```
#[derive(Copy, Clone, Debug)]
pub struct CallOperation(pub Option<CallCondition>);

impl Disassemble for CallOperation {
    fn disassemble(&self, cart: &Cartridge, offset: usize) -> crate::Result<AssemblyInstruction> {
        let address = word_to_u16((cart.data[offset], cart.data[offset + 1]));
        let builder = AssemblyInstructionBuilder::new()
            .with_command("CALL")
            .with_size(3);

        match self.0 {
            Some(condition) => builder.with_arg(condition),
            None => builder,
        }
        .with_arg(format!("${:0>4X}", address))
        .build()
    }

    fn describe(&self) -> crate::Result<AssemblyInstruction> {
        let mut builder = AssemblyInstructionBuilder::new()
            .with_command("CALL")
            .with_size(3);

        builder = match self.0 {
            Some(condition) => builder.with_arg(condition).with_arg("a16"),
            None => builder.with_arg("a16"),
        };

        builder.build()
    }
}

impl Operation for CallOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        let target = state.cpu.get16(WideRegister::SP) - 2;
        let program_counter = state.cpu.get16(WideRegister::PC);

        let conditional_passed = self
            .0
            .map(|condition| match condition {
                CallCondition::Zero => state.cpu.has_flag(Flags::ZERO),
                CallCondition::NotZero => !state.cpu.has_flag(Flags::ZERO),
                CallCondition::Carry => state.cpu.has_flag(Flags::CARRY),
                CallCondition::NotCarry => !state.cpu.has_flag(Flags::CARRY),
            })
            .unwrap_or(true);

        if conditional_passed {
            state.mmu[target] = (program_counter >> 8) as u8;
            state.mmu[target + 1] = program_counter as u8;

            let address = word_to_u16(state.read_word()?);

            state.cpu.set16(WideRegister::SP, target);
            state.cpu.set16(WideRegister::PC, address);
            state.cpu.increment_clock(24);
        } else {
            state.cpu.increment_clock(12);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod disassemble {
        use super::*;

        #[test]
        fn it_describes_properly() {
            let op = CallOperation(None);
            let assembly = op.describe().unwrap();

            assert_eq!("CALL a16", assembly.to_string());

            let op = CallOperation(Some(CallCondition::Zero));
            let assembly = op.describe().unwrap();

            assert_eq!("CALL Z,a16", assembly.to_string());

            let op = CallOperation(Some(CallCondition::NotZero));
            let assembly = op.describe().unwrap();

            assert_eq!("CALL NZ,a16", assembly.to_string());

            let op = CallOperation(Some(CallCondition::Carry));
            let assembly = op.describe().unwrap();

            assert_eq!("CALL C,a16", assembly.to_string());

            let op = CallOperation(Some(CallCondition::NotCarry));
            let assembly = op.describe().unwrap();

            assert_eq!("CALL NC,a16", assembly.to_string());
        }

        #[test]
        fn it_disassembles_properly() {
            let cartridge = Cartridge {
                data: vec![0xBE, 0xEF],
                ..Default::default()
            };

            let op = CallOperation(None);
            let assembly = op.disassemble(&cartridge, 0).unwrap();

            assert_eq!("CALL $BEEF", assembly.to_string());

            let op = CallOperation(Some(CallCondition::Zero));
            let assembly = op.disassemble(&cartridge, 0).unwrap();

            assert_eq!("CALL Z,$BEEF", assembly.to_string());

            let op = CallOperation(Some(CallCondition::NotZero));
            let assembly = op.disassemble(&cartridge, 0).unwrap();

            assert_eq!("CALL NZ,$BEEF", assembly.to_string());

            let op = CallOperation(Some(CallCondition::Carry));
            let assembly = op.disassemble(&cartridge, 0).unwrap();

            assert_eq!("CALL C,$BEEF", assembly.to_string());

            let op = CallOperation(Some(CallCondition::NotCarry));
            let assembly = op.disassemble(&cartridge, 0).unwrap();

            assert_eq!("CALL NC,$BEEF", assembly.to_string());
        }
    }

    mod operation {
        use crate::system::WideRegister;

        use super::*;

        fn setup() -> State {
            let cartridge = Cartridge {
                data: vec![0; 0xFFFF],
                ..Default::default()
            };

            let mut state = State::default();
            state.cpu.set16(WideRegister::SP, 0xBEEF);
            state.cpu.set16(WideRegister::PC, 0xDEAD);
            state.load_cartridge(cartridge);
            state
        }

        #[test]
        fn it_updates_the_stack_pointer() {
            let mut state = setup();

            CallOperation(None).act(&mut state).unwrap();

            assert_eq!(0xBEED, state.cpu.get16(WideRegister::SP));
        }

        #[test]
        fn it_writes_the_program_counter_to_memory() {
            let mut state = setup();

            CallOperation(None).act(&mut state).unwrap();

            assert_eq!(0xDE, state.mmu[0xBEED]);
            assert_eq!(0xAD, state.mmu[0xBEEE]);
        }

        #[test]
        fn it_writes_the_new_program_counter() {
            let mut state = setup();
            state.mmu[0xDEAD] = 0x20;
            state.mmu[0xDEAE] = 0x20;

            CallOperation(None).act(&mut state).unwrap();

            assert_eq!(0x2020, state.cpu.get16(WideRegister::PC));
        }

        #[test]
        fn it_correctly_checks_the_conditional() {
            let conditions = [
                (CallCondition::Zero, Flags::ZERO, true),
                (CallCondition::NotZero, Flags::ZERO, false),
                (CallCondition::Carry, Flags::CARRY, true),
                (CallCondition::NotCarry, Flags::CARRY, false),
            ];

            for (condition, flag, is_set) in conditions.iter() {
                // Check that the call succeeds when the flag is in the correct state
                let mut state = setup();
                state.cpu.set_flag_value(*flag, *is_set);
                state.mmu[0xDEAD] = 0x20;
                state.mmu[0xDEAE] = 0x20;

                CallOperation(Some(*condition)).act(&mut state).unwrap();

                assert_eq!(0x2020, state.cpu.get16(WideRegister::PC));

                // Check that the call noops when the flag is in the incorrect state
                state = setup();
                state.cpu.set_flag_value(*flag, !is_set);
                state.mmu[0xDEAD] = 0x20;
                state.mmu[0xDEAE] = 0x20;

                CallOperation(Some(*condition)).act(&mut state).unwrap();

                assert_eq!(0xDEAD, state.cpu.get16(WideRegister::PC));
            }
        }
    }
}
