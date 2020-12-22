use crate::{
    assembly::{AssemblyInstruction, AssemblyInstructionBuilder, Disassemble},
    operations::Operation,
    system::{Flags, Register, WideRegister},
    Cartridge, State,
};

/// The target of the AND operation.
#[derive(Copy, Clone, Debug)]
pub enum AndTarget {
    /// Compare with the value in a register.
    Register(Register),
    /// Compare with the value at the address given by HL.
    Address,
    /// Compare with the immediate value.
    Immediate,
}

impl std::fmt::Display for AndTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AndTarget::Register(reg) => write!(f, "{}", reg),
            AndTarget::Address => write!(f, "(HL)"),
            AndTarget::Immediate => write!(f, "d8"),
        }
    }
}

/// Compares the given target with the contents of the A register.
///
/// # Opcode reference
/// ## Assembly definition
/// ```a
/// AND B
/// ```
///
/// ## Runtime
/// | Metric | Size |
/// |:-------|:-----|
/// | Length | 1*   |
/// | Cycles | 4**  |
///
/// \* `SUB d8` is two bytes long.
/// \*\* `SUB (HL)` and `SUB d8` are both 8 cycles.
///
/// ## Flags
/// | Flag       | Value |
/// |:-----------|:------|
/// | Zero       | Set   |
/// | Subtract   | 0     |
/// | Half-Carry | 1     |
/// | Carry      | 0     |
///
/// # Examples
/// ```rs
/// AndOperation(AndTarget::Register(Register::B)).act(&cartridge)?;
/// ```
#[derive(Copy, Clone, Debug)]
pub struct AndOperation(pub AndTarget);

impl Operation for AndOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        let left = state.cpu.get(Register::A);
        let right = match &self.0 {
            AndTarget::Register(reg) => state.cpu.get(*reg),
            AndTarget::Address => {
                let address = state.cpu.get16(WideRegister::HL);
                state.mmu[address]
            }
            AndTarget::Immediate => state.read_byte()?,
        };

        let result = left & right;

        state.cpu.set(Register::A, result);

        let cycle_count = match &self.0 {
            AndTarget::Register(_) => 4,
            _ => 8,
        };

        state.cpu.increment_clock(cycle_count);

        state.cpu.set_flag_value(Flags::ZERO, result == 0);
        state.cpu.clear_flag(Flags::SUBTRACTION);
        state.cpu.set_flag(Flags::HALF_CARRY);
        state.cpu.clear_flag(Flags::CARRY);

        Ok(())
    }
}

impl Disassemble for AndOperation {
    fn disassemble(
        &self,
        cartridge: &Cartridge,
        offset: usize,
    ) -> crate::Result<AssemblyInstruction> {
        match &self.0 {
            AndTarget::Register(_) | AndTarget::Address => self.describe(),
            AndTarget::Immediate => AssemblyInstructionBuilder::new()
                .with_command("AND")
                .with_arg(format!("${:0>2X}", cartridge.data[offset + 1]))
                .with_size(2)
                .build(),
        }
    }

    fn describe(&self) -> crate::Result<AssemblyInstruction> {
        AssemblyInstructionBuilder::new()
            .with_command("AND")
            .with_arg(self.0)
            .with_size(match &self.0 {
                AndTarget::Register(_) | AndTarget::Address => 1,
                AndTarget::Immediate => 2,
            })
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod operation {
        use super::*;

        #[test]
        fn it_correct_sets_the_accumulator() {
            let mut state = State::default();
            state.cpu.set(Register::A, 0b0000_0011);
            state.cpu.set(Register::B, 0b0000_0001);

            AndOperation(AndTarget::Register(Register::B))
                .act(&mut state)
                .expect("Couldn't complete the AND");

            assert_eq!(0b0000_0001, state.cpu.get(Register::A));
        }

        #[test]
        fn it_sets_the_zero_flag() {
            let mut state = State::default();

            AndOperation(AndTarget::Register(Register::B))
                .act(&mut state)
                .expect("Couldn't complete the AND");

            assert!(state.cpu.has_flag(Flags::ZERO));
        }

        #[test]
        fn it_clears_the_subtraction_flag() {
            let mut state = State::default();
            state.cpu.set_flag(Flags::SUBTRACTION);

            AndOperation(AndTarget::Register(Register::B))
                .act(&mut state)
                .unwrap();

            assert!(!state.cpu.has_flag(Flags::SUBTRACTION));
        }

        #[test]
        fn it_sets_the_half_carry_flag() {
            let mut state = State::default();

            AndOperation(AndTarget::Register(Register::B))
                .act(&mut state)
                .unwrap();

            assert!(state.cpu.has_flag(Flags::HALF_CARRY));
        }

        #[test]
        fn it_clears_the_carry_flag() {
            let mut state = State::default();
            state.cpu.set_flag(Flags::CARRY);

            AndOperation(AndTarget::Register(Register::B))
                .act(&mut state)
                .unwrap();

            assert!(!state.cpu.has_flag(Flags::CARRY));
        }
    }

    mod disassemble {
        use super::*;

        #[test]
        fn it_disassembles_correctly() {
            let mut cartridge = Cartridge::default();
            cartridge.data.push(0x00);
            cartridge.data.push(0x10);

            let operation = AndOperation(AndTarget::Register(Register::B));
            let instruction = operation.disassemble(&cartridge, 0).unwrap();

            assert_eq!("AND B", instruction.to_string());

            let operation = AndOperation(AndTarget::Address);
            let instruction = operation.disassemble(&cartridge, 0).unwrap();

            assert_eq!("AND (HL)", instruction.to_string());

            let operation = AndOperation(AndTarget::Immediate);
            let instruction = operation.disassemble(&cartridge, 0).unwrap();

            assert_eq!("AND $10", instruction.to_string());
        }

        #[test]
        fn it_describes_correctly() {
            let operation = AndOperation(AndTarget::Register(Register::B));
            let instruction = operation.describe().unwrap();

            assert_eq!("AND B", instruction.to_string());

            let operation = AndOperation(AndTarget::Address);
            let instruction = operation.describe().unwrap();

            assert_eq!("AND (HL)", instruction.to_string());

            let operation = AndOperation(AndTarget::Immediate);
            let instruction = operation.describe().unwrap();

            assert_eq!("AND d8", instruction.to_string());
        }
    }
}
