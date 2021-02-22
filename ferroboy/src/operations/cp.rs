use crate::{
    assembly::{AssemblyInstruction, AssemblyInstructionBuilder, Disassemble},
    operations::Operation,
    system::{Alu, Flags, Register, WideRegister},
    Cartridge, State,
};

/// The target of the CP operation.
#[derive(Copy, Clone, Debug)]
pub enum CpTarget {
    /// Compare the given register to the accumulator.
    Register(Register),
    /// Compare the value at a given address to the accumulator.
    Address,
    /// Compare the immediate byte with the accumulator.
    Immediate,
}

impl std::fmt::Display for CpTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CpTarget::Register(r) => write!(f, "{}", r),
            CpTarget::Address => write!(f, "(HL)"),
            CpTarget::Immediate => write!(f, "d8"),
        }
    }
}

/// Compares a given target to the accumulator and sets flags accordingly.
///
/// # Opcode reference
/// ## Assembly definition
/// ```a
/// CP B
/// ```
///
/// ## Runtime
/// | Metric | Size |
/// |:-------|:-----|
/// | Length | 1*   |
/// | Cycles | 4**  |
///
/// \* `CP d8` is 2 bytes
/// \*\* `CP (HL)` and `CP d8` are 8 cycles
///
/// ## Flags
/// | Flag        | Value |
/// |:------------|:------|
/// | Zero        | Set   |
/// | Subtraction | 1     |
/// | Half-Carry  | Set   |
/// | Carry       | Set   |
///
/// # Examples
/// ```rs
/// CpOperation(Register::B).act(&mut state).unwrap();
/// ```
#[derive(Copy, Clone, Debug)]
pub struct CpOperation(pub CpTarget);

impl CpOperation {
    fn cycle_count(&self) -> u64 {
        match &self.0 {
            CpTarget::Register(_) => 4,
            _ => 8,
        }
    }

    fn size(&self) -> u8 {
        match &self.0 {
            CpTarget::Immediate => 2,
            _ => 1,
        }
    }
}

impl Operation for CpOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        let left = state.cpu.get(Register::A);
        let right = match &self.0 {
            CpTarget::Register(r) => state.cpu.get(*r),
            CpTarget::Address => {
                let address = state.cpu.get16(WideRegister::Hl);
                state.mmu[address]
            }
            CpTarget::Immediate => state.read_byte()?,
        };

        let (result, carry, half_carry) = left.alu_sub(right);

        state.cpu.increment_clock(self.cycle_count());
        state.cpu.set_flag_value(Flags::ZERO, result == 0);
        state.cpu.set_flag(Flags::SUBTRACTION);
        state.cpu.set_flag_value(Flags::HALF_CARRY, half_carry);
        state.cpu.set_flag_value(Flags::CARRY, carry);

        Ok(())
    }
}

impl Disassemble for CpOperation {
    fn disassemble(
        &self,
        cartridge: &Cartridge,
        offset: usize,
    ) -> crate::Result<AssemblyInstruction> {
        AssemblyInstructionBuilder::new()
            .with_command("CP")
            .with_arg(match &self.0 {
                CpTarget::Register(r) => r.to_string(),
                CpTarget::Address => String::from("(HL)"),
                CpTarget::Immediate => {
                    let data = cartridge.data[offset + 1];
                    format!("${:0<2X}", data)
                }
            })
            .with_size(self.size())
            .build()
    }

    fn describe(&self) -> crate::Result<AssemblyInstruction> {
        AssemblyInstructionBuilder::new()
            .with_command("CP")
            .with_arg(self.0)
            .with_size(self.size())
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod disassemble {
        use super::*;

        #[test]
        fn it_describes_correctly() {
            let operation = CpOperation(CpTarget::Register(Register::B));
            let instruction = operation.describe().unwrap();

            assert_eq!("CP B", instruction.to_string());

            let operation = CpOperation(CpTarget::Address);
            let instruction = operation.describe().unwrap();

            assert_eq!("CP (HL)", instruction.to_string());

            let operation = CpOperation(CpTarget::Immediate);
            let instruction = operation.describe().unwrap();

            assert_eq!("CP d8", instruction.to_string());
        }

        #[test]
        fn it_disassembles_correctly() {
            let cartridge = Cartridge {
                data: vec![0xDE, 0xAD, 0xBE, 0xEF],
                ..Default::default()
            };

            let operation = CpOperation(CpTarget::Register(Register::B));
            let instruction = operation.disassemble(&cartridge, 0).unwrap();

            assert_eq!("CP B", instruction.to_string());

            let operation = CpOperation(CpTarget::Address);
            let instruction = operation.disassemble(&cartridge, 1).unwrap();

            assert_eq!("CP (HL)", instruction.to_string());

            let operation = CpOperation(CpTarget::Immediate);
            let instruction = operation.disassemble(&cartridge, 2).unwrap();

            assert_eq!("CP $EF", instruction.to_string());
        }
    }

    mod operation {
        use super::*;

        #[test]
        fn it_doesnt_alter_register_values() {
            let mut state = State::default();
            state.cpu.set(Register::A, 0b0000_0001);
            state.cpu.set(Register::B, 0b0000_0001);

            CpOperation(CpTarget::Register(Register::B))
                .act(&mut state)
                .unwrap();

            assert_eq!(0b0000_0001, state.cpu.get(Register::A));
        }

        #[test]
        fn it_sets_the_zero_flag() {
            let mut state = State::default();
            state.cpu.set(Register::A, 0b0000_0001);
            state.cpu.set(Register::B, 0b0000_0001);

            CpOperation(CpTarget::Register(Register::B))
                .act(&mut state)
                .unwrap();

            assert!(state.cpu.has_flag(Flags::ZERO));
        }

        #[test]
        fn it_sets_the_subtraction_flag() {
            let mut state = State::default();

            CpOperation(CpTarget::Register(Register::B))
                .act(&mut state)
                .unwrap();

            assert!(state.cpu.has_flag(Flags::SUBTRACTION));
        }

        #[test]
        fn it_sets_the_half_carry_flag() {
            let mut state = State::default();
            state.cpu.set(Register::A, 0b0001_0000);
            state.cpu.set(Register::B, 0b0000_1000);

            CpOperation(CpTarget::Register(Register::B))
                .act(&mut state)
                .unwrap();

            assert!(state.cpu.has_flag(Flags::HALF_CARRY));
        }

        #[test]
        fn it_sets_the_carry_flag() {
            let mut state = State::default();
            state.cpu.set(Register::A, 0b0000_0001);
            state.cpu.set(Register::B, 0b0000_0010);

            CpOperation(CpTarget::Register(Register::B))
                .act(&mut state)
                .unwrap();

            assert!(state.cpu.has_flag(Flags::CARRY));
        }
    }
}
