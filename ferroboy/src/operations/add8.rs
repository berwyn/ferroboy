use crate::assembly::{AssemblyInstruction, AssemblyInstructionBuilder, Disassemble};
use crate::extensions::{Carry, HalfCarry};
use crate::operations::Operation;
use crate::state::State;
use crate::system::{Flags, Register};

/// Add the contents of one register to another
///
/// # Opcode Reference
/// ## Assembly definition
/// ```a
/// ADD A,B
/// ```
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
/// | Subtraction | 0     |
/// | Half-Carry  | Set   |
/// | Carry       | Set   |
///
/// # Examples
/// ```rs
/// let operation = Add8Operation(Register::A, Register::B);
/// operation.act(&mut state).unwrap();
/// ```
///
/// # Errors
/// - The operation may fail if a 16-bit register is provided for either argument.
#[derive(Clone, Copy, Debug)]
pub struct Add8Operation(pub Register, pub Register);

impl Operation for Add8Operation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        let left_hand = state.cpu.get(self.0);
        let right_hand = state.cpu.get(self.1);
        let new_value = left_hand.wrapping_add(right_hand);

        state.cpu.set(self.0, new_value);

        state.cpu.set_flag_value(Flags::ZERO, new_value == 0);
        state.cpu.clear_flag(Flags::SUBTRACTION);

        state
            .cpu
            .set_flag_value(Flags::HALF_CARRY, left_hand.half_carry(right_hand));

        state
            .cpu
            .set_flag_value(Flags::CARRY, left_hand.carry(right_hand));

        state.cpu.increment_clock(4);

        Ok(())
    }
}

impl Disassemble for Add8Operation {
    fn disassemble(&self, _: &mut State) -> crate::Result<AssemblyInstruction> {
        AssemblyInstructionBuilder::new()
            .with_command("ADD")
            .with_arg(self.0)
            .with_arg(self.1)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_disassembles_correctly() {
        let add = Add8Operation(Register::A, Register::B);
        let instruction = add.disassemble(&mut State::default()).unwrap();

        assert_eq!("ADD A,B", instruction.to_string());
    }

    #[test]
    fn it_adds_two_registers() -> crate::Result<()> {
        let mut state = State::default();
        state.cpu.set(Register::A, 0x10);
        state.cpu.set(Register::B, 0x06);

        Add8Operation(Register::A, Register::B).act(&mut state)
    }

    #[test]
    fn it_clears_the_subtraction_flag() {
        let mut state = State::default();
        state.cpu.set_flag(Flags::SUBTRACTION);

        Add8Operation(Register::A, Register::B)
            .act(&mut state)
            .unwrap();

        assert_eq!(false, state.cpu.has_flag(Flags::SUBTRACTION))
    }

    #[test]
    fn it_sets_the_half_carry_flag() {
        let mut state = State::default();
        state.cpu.set(Register::A, 0x18);
        state.cpu.set(Register::B, 0x08);

        Add8Operation(Register::A, Register::B)
            .act(&mut state)
            .unwrap();

        assert_eq!(0x20, state.cpu.get(Register::A));
        assert!(state.cpu.has_flag(Flags::HALF_CARRY));
    }

    #[test]
    fn it_sets_the_carry_flag() {
        let mut state = State::default();
        state.cpu.set(Register::A, 0xA0);
        state.cpu.set(Register::B, 0x60);

        Add8Operation(Register::A, Register::B)
            .act(&mut state)
            .unwrap();

        assert_eq!(0x00, state.cpu.get(Register::A));
        assert!(state.cpu.has_flag(Flags::CARRY));
    }

    #[test]
    fn it_sets_the_zero_flag() {
        let mut state = State::default();
        state.cpu.set(Register::A, 0xFF);
        state.cpu.set(Register::B, 0x01);

        Add8Operation(Register::A, Register::B)
            .act(&mut state)
            .unwrap();

        assert_eq!(0x00, state.cpu.get(Register::A));
        assert!(state.cpu.has_flag(Flags::ZERO));
    }
}
