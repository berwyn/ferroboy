use crate::{
    assembly::{AssemblyInstruction, AssemblyInstructionBuilder, Disassemble},
    helpers::word_to_u16,
    operations::Operation,
    system::{Flags, WideRegister},
    State,
};

/// The condition to check before returning.
#[derive(Copy, Clone, Debug)]
pub enum RetCondition {
    Zero,
    NotZero,
    Carry,
    NotCarry,
}

impl std::fmt::Display for RetCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RetCondition::Zero => "Z",
                RetCondition::NotZero => "NZ",
                RetCondition::Carry => "C",
                RetCondition::NotCarry => "NC",
            }
        )
    }
}

/// Sets the program counter to the address stored in the stack pointer, if the condition check succeeds.
///
/// # Opcode reference
/// ## Assembly definition
/// ```a
/// RET ; No condition
/// RET Z ; With a condition
/// ```
///
/// ## Runtime
/// | Metric | Size |
/// |:-------|:-----|
/// | Length | 1    |
/// | Cycles | 16\* |
///
/// \* `RET` is 16 cycles, however conditional `RET` with `Z`/`NZ`/etc is 8 cycles if the check fails and 20 cycles if
/// the check succeeds.
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
/// RetOperation(None).act(&mut state).unwrap();
/// ```
#[derive(Copy, Clone, Debug)]
pub struct RetOperation(pub Option<RetCondition>);

impl Operation for RetOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        let target = state.cpu.get16(WideRegister::SP);
        let (high, low) = (state.mmu[target], state.mmu[target + 1]);
        let address = word_to_u16((high, low));

        let is_zero = state.cpu.has_flag(Flags::ZERO);
        let is_carry = state.cpu.has_flag(Flags::CARRY);

        let cycle_count = match self.0 {
            None => 16,
            Some(condition) => match (condition, is_zero, is_carry) {
                (RetCondition::Zero, true, _) => 20,
                (RetCondition::Zero, false, _) => 8,
                (RetCondition::NotZero, true, _) => 20,
                (RetCondition::NotZero, false, _) => 8,
                (RetCondition::Carry, _, true) => 20,
                (RetCondition::Carry, _, false) => 8,
                (RetCondition::NotCarry, _, true) => 20,
                (RetCondition::NotCarry, _, false) => 8,
            },
        };

        state.cpu.set_clock(|clock| clock + cycle_count);

        if let Some(condition) = &self.0 {
            match (condition, is_zero, is_carry) {
                (RetCondition::Zero, false, _) => return Ok(()),
                (RetCondition::NotZero, true, _) => return Ok(()),
                (RetCondition::Carry, _, false) => return Ok(()),
                (RetCondition::NotCarry, _, true) => return Ok(()),
                _ => {}
            }
        }

        state.cpu.set16(WideRegister::PC, address);
        state.cpu.set16(WideRegister::SP, target + 2);

        Ok(())
    }
}

impl Disassemble for RetOperation {
    fn disassemble(&self, _: &crate::Cartridge, _: usize) -> crate::Result<AssemblyInstruction> {
        self.describe()
    }

    fn describe(&self) -> crate::Result<AssemblyInstruction> {
        let mut builder = AssemblyInstructionBuilder::new().with_command("RET");

        if self.0.is_some() {
            builder = builder.with_arg(self.0.unwrap());
        }

        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod disassemble {
        use crate::Cartridge;

        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn it_describes_properly() {
            let instruction = RetOperation(None).describe().unwrap();
            assert_eq!("RET", instruction.to_string());

            let instruction = RetOperation(Some(RetCondition::Zero)).describe().unwrap();
            assert_eq!("RET Z", instruction.to_string());

            let instruction = RetOperation(Some(RetCondition::NotZero))
                .describe()
                .unwrap();
            assert_eq!("RET NZ", instruction.to_string());

            let instruction = RetOperation(Some(RetCondition::Carry)).describe().unwrap();
            assert_eq!("RET C", instruction.to_string());

            let instruction = RetOperation(Some(RetCondition::NotCarry))
                .describe()
                .unwrap();
            assert_eq!("RET NC", instruction.to_string());
        }

        #[test]
        fn it_disassembles_properly() {
            let cartridge = Cartridge::default();

            let instruction = RetOperation(None).disassemble(&cartridge, 0).unwrap();
            assert_eq!("RET", instruction.to_string());

            let instruction = RetOperation(Some(RetCondition::Zero))
                .disassemble(&cartridge, 0)
                .unwrap();
            assert_eq!("RET Z", instruction.to_string());

            let instruction = RetOperation(Some(RetCondition::NotZero))
                .disassemble(&cartridge, 0)
                .unwrap();
            assert_eq!("RET NZ", instruction.to_string());

            let instruction = RetOperation(Some(RetCondition::Carry))
                .disassemble(&cartridge, 0)
                .unwrap();
            assert_eq!("RET C", instruction.to_string());

            let instruction = RetOperation(Some(RetCondition::NotCarry))
                .disassemble(&cartridge, 0)
                .unwrap();
            assert_eq!("RET NC", instruction.to_string());
        }
    }

    mod operation {
        use super::*;
        use pretty_assertions::assert_eq;

        fn create_state() -> State {
            let mut state = State::default();
            state.cpu.set16(WideRegister::PC, 0xDEAD);
            state.cpu.set16(WideRegister::SP, 0xBEEF);
            state.mmu[0xBEEF] = 0xFA;
            state.mmu[0xBEF0] = 0xCE;
            state
        }

        #[test]
        fn it_sets_the_program_counter() {
            let mut state = create_state();

            RetOperation(None).act(&mut state).unwrap();

            assert_eq!(
                0xFACE,
                state.cpu.get16(WideRegister::PC),
                "It sets the program counter"
            );

            assert_eq!(
                0xBEF1,
                state.cpu.get16(WideRegister::SP),
                "It sets the stack pointer"
            );
        }

        #[test]
        fn it_respects_the_zero_flag() {
            let mut state = create_state();
            state.cpu.clear_flag(Flags::ZERO);

            RetOperation(Some(RetCondition::Zero))
                .act(&mut state)
                .unwrap();

            assert_eq!(
                0xDEAD,
                state.cpu.get16(WideRegister::PC),
                "It leaves the program counter alone"
            );

            assert_eq!(
                0xBEEF,
                state.cpu.get16(WideRegister::SP),
                "It leaves the stack pointer alone"
            );

            state.cpu.set_flag(Flags::ZERO);

            RetOperation(Some(RetCondition::Zero))
                .act(&mut state)
                .unwrap();

            assert_eq!(
                0xFACE,
                state.cpu.get16(WideRegister::PC),
                "It leaves the program counter alone"
            );

            assert_eq!(
                0xBEF1,
                state.cpu.get16(WideRegister::SP),
                "It leaves the stack pointer alone"
            );
        }

        #[test]
        fn it_respects_the_not_zero_flag() {
            let mut state = create_state();
            state.cpu.set_flag(Flags::ZERO);

            RetOperation(Some(RetCondition::NotZero))
                .act(&mut state)
                .unwrap();

            assert_eq!(
                0xDEAD,
                state.cpu.get16(WideRegister::PC),
                "It leaves the program counter alone"
            );

            assert_eq!(
                0xBEEF,
                state.cpu.get16(WideRegister::SP),
                "It leaves the stack pointer alone"
            );

            state.cpu.clear_flag(Flags::ZERO);

            RetOperation(Some(RetCondition::NotZero))
                .act(&mut state)
                .unwrap();

            assert_eq!(
                0xFACE,
                state.cpu.get16(WideRegister::PC),
                "It leaves the program counter alone"
            );

            assert_eq!(
                0xBEF1,
                state.cpu.get16(WideRegister::SP),
                "It leaves the stack pointer alone"
            );
        }

        #[test]
        fn it_respects_the_carry_flag() {
            let mut state = create_state();
            state.cpu.clear_flag(Flags::CARRY);

            RetOperation(Some(RetCondition::Carry))
                .act(&mut state)
                .unwrap();

            assert_eq!(
                0xDEAD,
                state.cpu.get16(WideRegister::PC),
                "It leaves the program counter alone"
            );

            assert_eq!(
                0xBEEF,
                state.cpu.get16(WideRegister::SP),
                "It leaves the stack pointer alone"
            );

            state.cpu.set_flag(Flags::CARRY);

            RetOperation(Some(RetCondition::Carry))
                .act(&mut state)
                .unwrap();

            assert_eq!(
                0xFACE,
                state.cpu.get16(WideRegister::PC),
                "It leaves the program counter alone"
            );

            assert_eq!(
                0xBEF1,
                state.cpu.get16(WideRegister::SP),
                "It leaves the stack pointer alone"
            );
        }

        #[test]
        fn it_respects_the_not_carry_flag() {
            let mut state = create_state();
            state.cpu.set_flag(Flags::CARRY);

            RetOperation(Some(RetCondition::NotCarry))
                .act(&mut state)
                .unwrap();

            assert_eq!(
                0xDEAD,
                state.cpu.get16(WideRegister::PC),
                "It leaves the program counter alone"
            );

            assert_eq!(
                0xBEEF,
                state.cpu.get16(WideRegister::SP),
                "It leaves the stack pointer alone"
            );

            state.cpu.clear_flag(Flags::CARRY);

            RetOperation(Some(RetCondition::NotCarry))
                .act(&mut state)
                .unwrap();

            assert_eq!(
                0xFACE,
                state.cpu.get16(WideRegister::PC),
                "It leaves the program counter alone"
            );

            assert_eq!(
                0xBEF1,
                state.cpu.get16(WideRegister::SP),
                "It leaves the stack pointer alone"
            );
        }
    }
}
