use crate::{
    assembly::{AssemblyInstruction, AssemblyInstructionBuilder, Disassemble},
    operations::Operation,
    system::{Flags, Register, WideRegister},
    Cartridge, State,
};

/// The target of the OR operation.
#[derive(Copy, Clone, Debug)]
pub enum XorTarget {
    /// Compare with the value in a register.
    Register(Register),
    /// Compare with the value at the address given by HL.
    Address,
    /// Compare with the immediate value.
    Immediate,
}

impl std::fmt::Display for XorTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            XorTarget::Register(reg) => write!(f, "{}", reg),
            XorTarget::Address => write!(f, "(HL)"),
            XorTarget::Immediate => write!(f, "d8"),
        }
    }
}

/// Compares the given target with the contents of the A register.
///
/// # Opcode reference
/// ## Assembly definition
/// ```a
/// OR B
/// ```
///
/// ## Runtime
/// | Metric | Size |
/// |:-------|:-----|
/// | Length | 1*   |
/// | Cycles | 4**  |
///
/// \* `SUB d8` is 2 bytes
/// \*\* `SUB (HL)` and `SUB d8` are 8 cycles
///
/// ## Flags
/// | Flag        | Value |
/// |:------------|:------|
/// | Zero        | Set   |
/// | Subtraction | 0     |
/// | Half-Carry  | 0     |
/// | Carry       | 0     |
///
/// # Example
/// ```rs
/// OrOperation(OrTarget::Register(Register::B)).act(&mut state)?;
/// ```
#[derive(Copy, Clone, Debug)]
pub struct XorOperation(pub XorTarget);

impl XorOperation {
    fn cycle_count(&self) -> u64 {
        match &self.0 {
            XorTarget::Register(_) => 4,
            XorTarget::Address | XorTarget::Immediate => 8,
        }
    }

    fn size(&self) -> u8 {
        match &self.0 {
            XorTarget::Register(_) | XorTarget::Address => 1,
            XorTarget::Immediate => 2,
        }
    }
}

impl Operation for XorOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        let left = state.cpu.get(Register::A);
        let right = match &self.0 {
            XorTarget::Register(reg) => state.cpu.get(*reg),
            XorTarget::Address => {
                let address = state.cpu.get16(WideRegister::HL);
                state.mmu[address]
            }
            XorTarget::Immediate => state.read_byte()?,
        };

        let result = left ^ right;

        state.cpu.set(Register::A, result);
        state.cpu.increment_clock(self.cycle_count());
        state.cpu.set_flag_value(Flags::ZERO, result == 0);
        state.cpu.clear_flag(Flags::SUBTRACTION);
        state.cpu.clear_flag(Flags::HALF_CARRY);
        state.cpu.clear_flag(Flags::CARRY);

        Ok(())
    }
}

impl Disassemble for XorOperation {
    fn disassemble(
        &self,
        cartridge: &Cartridge,
        offset: usize,
    ) -> crate::Result<AssemblyInstruction> {
        match &self.0 {
            XorTarget::Register(_) | XorTarget::Address => self.describe(),
            XorTarget::Immediate => AssemblyInstructionBuilder::new()
                .with_command("XOR")
                .with_arg(format!("${:0>2X}", cartridge.data[offset + 1]))
                .with_size(2)
                .build(),
        }
    }

    fn describe(&self) -> crate::Result<AssemblyInstruction> {
        AssemblyInstructionBuilder::new()
            .with_command("XOR")
            .with_arg(self.0)
            .with_size(self.size())
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod operation {
        use super::*;

        #[test]
        fn it_sets_the_accumulator_correctly() {
            let mut state = State::default();
            state.cpu.set(Register::A, 0b0000_0011);
            state.cpu.set(Register::B, 0b0000_0001);

            XorOperation(XorTarget::Register(Register::B))
                .act(&mut state)
                .unwrap();

            assert_eq!(0b0000_0010, state.cpu.get(Register::A));
        }

        #[test]
        fn it_sets_the_zero_flag() {
            let mut state = State::default();

            XorOperation(XorTarget::Register(Register::B))
                .act(&mut state)
                .unwrap();

            assert!(state.cpu.has_flag(Flags::ZERO));
        }

        #[test]
        fn it_clears_the_subtraction_flag() {
            let mut state = State::default();
            state.cpu.set_flag(Flags::SUBTRACTION);

            XorOperation(XorTarget::Register(Register::B))
                .act(&mut state)
                .unwrap();

            assert!(!state.cpu.has_flag(Flags::SUBTRACTION));
        }

        #[test]
        fn it_clears_the_half_carry_flag() {
            let mut state = State::default();
            state.cpu.set_flag(Flags::HALF_CARRY);

            XorOperation(XorTarget::Register(Register::B))
                .act(&mut state)
                .unwrap();

            assert!(!state.cpu.has_flag(Flags::HALF_CARRY));
        }

        #[test]
        fn it_clears_the_carry_flag() {
            let mut state = State::default();
            state.cpu.set_flag(Flags::CARRY);

            XorOperation(XorTarget::Register(Register::B))
                .act(&mut state)
                .unwrap();

            assert!(!state.cpu.has_flag(Flags::CARRY));
        }
    }

    mod disassemble {
        use super::*;

        #[test]
        fn it_describes_property() {
            let operation = XorOperation(XorTarget::Register(Register::B));
            let instruction = operation.describe().unwrap();

            assert_eq!("XOR B", instruction.to_string());

            let operation = XorOperation(XorTarget::Address);
            let instruction = operation.describe().unwrap();

            assert_eq!("XOR (HL)", instruction.to_string());

            let operation = XorOperation(XorTarget::Immediate);
            let instruction = operation.describe().unwrap();

            assert_eq!("XOR d8", instruction.to_string());
        }

        #[test]
        fn it_disassembles_correctly() {
            let mut cartridge = Cartridge::default();
            cartridge.data.push(0x00);
            cartridge.data.push(0x10);

            let operation = XorOperation(XorTarget::Register(Register::B));
            let instruction = operation.disassemble(&cartridge, 0).unwrap();

            assert_eq!("XOR B", instruction.to_string());

            let operation = XorOperation(XorTarget::Address);
            let instruction = operation.disassemble(&cartridge, 0).unwrap();

            assert_eq!("XOR (HL)", instruction.to_string());

            let operation = XorOperation(XorTarget::Immediate);
            let instruction = operation.disassemble(&cartridge, 0).unwrap();

            assert_eq!("XOR $10", instruction.to_string());
        }
    }
}
