use crate::{
    assembly::{AssemblyInstruction, AssemblyInstructionBuilder, Disassemble},
    operations::Operation,
    system::{Alu, Flags, Register, WideRegister},
    Cartridge, State,
};

/// The target of the SUB operation.
#[derive(Copy, Clone, Debug)]
pub enum SubTarget {
    /// Subtract a given register from the accumulator.
    Register(Register),
    /// Subtract the value at the address given by HL.
    Address,
    /// Subtract the 8-bit immediate value.
    Immediate,
}

impl std::fmt::Display for SubTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubTarget::Register(r) => write!(f, "{}", r),
            SubTarget::Address => write!(f, "(HL)"),
            SubTarget::Immediate => write!(f, "d8"),
        }
    }
}

/// Subtracts the given register from the A register.
///
/// # Opcode reference
/// ## Assembly definition
/// ```a
/// SUB B
/// ```
///
/// ## Runtime
/// | Metric | Size |
/// |:-------|:-----|
/// | Length | 1    |
/// | Cycles | 4    |
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
/// SubOperation(Register::B).act(&mut state).unwrap();
/// ```
#[derive(Copy, Clone, Debug)]
pub struct SubOperation(pub SubTarget);

impl Operation for SubOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        let left = state.cpu.get(Register::A);
        let right = match &self.0 {
            SubTarget::Register(reg) => state.cpu.get(*reg),
            SubTarget::Address => {
                let address = state.cpu.get16(WideRegister::Hl);
                state.mmu[address]
            }
            SubTarget::Immediate => state.read_byte()?,
        };

        let (new_value, carry, half_carry) = left.alu_sub(right);

        let cycle_count = match &self.0 {
            SubTarget::Register(_) => 4,
            _ => 8,
        };

        state.cpu.set(Register::A, new_value);
        state.cpu.increment_clock(cycle_count);

        state.cpu.set_flag_value(Flags::ZERO, new_value == 0);
        state.cpu.set_flag(Flags::SUBTRACTION);
        state.cpu.set_flag_value(Flags::HALF_CARRY, half_carry);
        state.cpu.set_flag_value(Flags::CARRY, carry);

        Ok(())
    }
}

impl Disassemble for SubOperation {
    fn disassemble(
        &self,
        cartridge: &Cartridge,
        offset: usize,
    ) -> crate::Result<AssemblyInstruction> {
        match self.0 {
            SubTarget::Register(_) | SubTarget::Address => self.describe(),
            SubTarget::Immediate => {
                let immediate = cartridge.data[offset + 1];

                AssemblyInstructionBuilder::new()
                    .with_command("SUB")
                    .with_arg(format!("${:0>2X}", immediate))
                    .with_size(2)
                    .build()
            }
        }
    }

    fn describe(&self) -> crate::Result<AssemblyInstruction> {
        AssemblyInstructionBuilder::new()
            .with_command("SUB")
            .with_arg(self.0)
            .with_size(match &self.0 {
                SubTarget::Register(_) | SubTarget::Address => 1,
                SubTarget::Immediate => 2,
            })
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_disassembles_correctly() {
        let mut cartridge = Cartridge::default();
        cartridge.data.push(0x00);
        cartridge.data.push(0x00);

        let operation = SubOperation(SubTarget::Register(Register::B));
        let instruction = operation.disassemble(&cartridge, 0).unwrap();

        assert_eq!("SUB B", instruction.to_string(), "With a register target");

        let operation = SubOperation(SubTarget::Address);
        let instruction = operation.disassemble(&cartridge, 0).unwrap();

        assert_eq!(
            "SUB (HL)",
            instruction.to_string(),
            "With an address target"
        );

        let operation = SubOperation(SubTarget::Immediate);
        let instruction = operation.disassemble(&cartridge, 0).unwrap();

        assert_eq!(
            "SUB $00",
            instruction.to_string(),
            "With an immediate target"
        );
    }

    #[test]
    fn it_subtracts_from_the_accumulator() {
        let mut state = State::default();
        state.cpu.set(Register::A, 0xF0);
        state.cpu.set(Register::B, 0x07);

        SubOperation(SubTarget::Register(Register::B))
            .act(&mut state)
            .unwrap();

        assert_eq!(0xE9, state.cpu.get(Register::A));
    }

    #[test]
    fn it_sets_the_zero_flag() {
        let mut state = State::default();
        state.cpu.set(Register::A, 0x07);
        state.cpu.set(Register::B, 0x07);

        SubOperation(SubTarget::Register(Register::B))
            .act(&mut state)
            .unwrap();

        assert_eq!(0x00, state.cpu.get(Register::A));
        assert!(state.cpu.has_flag(Flags::ZERO));
    }

    #[test]
    fn it_sets_the_subtraction_flag() {
        let mut state = State::default();

        SubOperation(SubTarget::Register(Register::B))
            .act(&mut state)
            .unwrap();

        assert!(state.cpu.has_flag(Flags::SUBTRACTION));
    }

    #[test]
    fn it_sets_the_half_carry_flag() {
        let mut state = State::default();
        state.cpu.set(Register::A, 0x10);
        state.cpu.set(Register::B, 0x01);

        SubOperation(SubTarget::Register(Register::B))
            .act(&mut state)
            .unwrap();

        assert!(state.cpu.has_flag(Flags::HALF_CARRY));
    }

    #[test]
    fn it_sets_the_carry_flag() {
        let mut state = State::default();
        state.cpu.set(Register::B, 0x01);

        SubOperation(SubTarget::Register(Register::B))
            .act(&mut state)
            .unwrap();

        assert!(state.cpu.has_flag(Flags::CARRY));
    }
}
