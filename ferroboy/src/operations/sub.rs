use crate::{
    assembly::{AssemblyInstruction, AssemblyInstructionBuilder, Disassemble},
    operations::Operation,
    system::{Flags, Register, ALU},
    Cartridge, State,
};

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
#[derive(Clone, Copy, Debug)]
pub struct SubOperation(pub Register);

impl Operation for SubOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        let left = state.cpu.get(Register::A);
        let right = state.cpu.get(self.0);
        let (new_value, carry, half_carry) = left.alu_sub(right);

        state.cpu.set(Register::A, new_value);
        state.cpu.increment_clock(4);

        state.cpu.set_flag_value(Flags::ZERO, new_value == 0);
        state.cpu.set_flag(Flags::SUBTRACTION);
        state.cpu.set_flag_value(Flags::HALF_CARRY, half_carry);
        state.cpu.set_flag_value(Flags::CARRY, carry);

        Ok(())
    }
}

impl Disassemble for SubOperation {
    fn disassemble(&self, _: &Cartridge, _: usize) -> crate::Result<AssemblyInstruction> {
        self.describe()
    }

    fn describe(&self) -> crate::Result<AssemblyInstruction> {
        AssemblyInstructionBuilder::new()
            .with_command("SUB")
            .with_arg(self.0)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_disassembles_correctly() {
        let operation = SubOperation(Register::B);
        let instruction = operation.disassemble(&Cartridge::default(), 0).unwrap();

        assert_eq!("SUB B", instruction.to_string());
    }

    #[test]
    fn it_subtracts_from_the_accumulator() {
        let mut state = State::default();
        state.cpu.set(Register::A, 0xF0);
        state.cpu.set(Register::B, 0x07);

        SubOperation(Register::B).act(&mut state).unwrap();

        assert_eq!(0xE9, state.cpu.get(Register::A));
    }

    #[test]
    fn it_sets_the_zero_flag() {
        let mut state = State::default();
        state.cpu.set(Register::A, 0x07);
        state.cpu.set(Register::B, 0x07);

        SubOperation(Register::B).act(&mut state).unwrap();

        assert_eq!(0x00, state.cpu.get(Register::A));
        assert!(state.cpu.has_flag(Flags::ZERO));
    }

    #[test]
    fn it_sets_the_subtraction_flag() {
        let mut state = State::default();

        SubOperation(Register::B).act(&mut state).unwrap();

        assert!(state.cpu.has_flag(Flags::SUBTRACTION));
    }

    #[test]
    fn it_sets_the_half_carry_flag() {
        let mut state = State::default();
        state.cpu.set(Register::A, 0x10);
        state.cpu.set(Register::B, 0x01);

        SubOperation(Register::B).act(&mut state).unwrap();

        assert!(state.cpu.has_flag(Flags::HALF_CARRY));
    }

    #[test]
    fn it_sets_the_carry_flag() {
        let mut state = State::default();
        state.cpu.set(Register::B, 0x01);

        SubOperation(Register::B).act(&mut state).unwrap();

        assert!(state.cpu.has_flag(Flags::CARRY));
    }
}
