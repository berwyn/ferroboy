use crate::{
    assembly::{AssemblyInstruction, AssemblyInstructionBuilder, Disassemble},
    operations::Operation,
    state::State,
    system::{Cartridge, Flags, WideRegister, ALU},
};

/*
 * 0x09 is ADD HL,BC
 * 0x19 is ADD HL,DE
 * 0x29 is ADD HL,BC
 * 0x39 is ADD HL,HL
 * 0x49 is ADD HL,SP
 * Since HL is the only valid target, and all the right-hand args are
 * 16-bit registers, we can probably simplify this and keep all state
 * reads and writes here
 */

/// Adds the contents of one 16-bit register to another
///
/// # Opcode Reference
/// ## Assembly definition
/// ```a
/// ADD HL,SP
/// ```
///
/// ## Runtime
/// | Metric | Size |
/// |:-------|:-----|
/// | Length | 1    |
/// | Cycles | 8    |
///
/// ## Flags
/// | Flag        | Value        |
/// |:------------|:-------------|
/// | Zero        | Not Affected |
/// | Subtraction | 0            |
/// | Half-Carry  | Set          |
/// | Carry       | Set          |
///
/// # Examples
/// ```rs
/// let operation = Add16Operation(Register::HL, Register::SP);
/// operation.act(&mut state).unwrap();
/// ```
///
/// # Errors
/// - The operation may fail if an 8-bit register is provided.
#[derive(Clone, Copy, Debug)]
pub struct Add16Operation(WideRegister);

impl Operation for Add16Operation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        if WideRegister::PC.eq(&self.0) {
            return Err("Cannot use PC in ADD".into());
        }

        let target = state.cpu.get16(WideRegister::HL);
        let source = state.cpu.get16(self.0);
        let (new_value, carry, half_carry) = target.alu_add(source);

        state.cpu.set16(WideRegister::HL, new_value);
        state.cpu.increment_clock(8);

        state.cpu.clear_flag(Flags::SUBTRACTION);
        state.cpu.set_flag_value(Flags::HALF_CARRY, half_carry);
        state.cpu.set_flag_value(Flags::CARRY, carry);

        Ok(())
    }
}

impl Disassemble for Add16Operation {
    fn disassemble(&self, _: &Cartridge, _: usize) -> crate::Result<AssemblyInstruction> {
        AssemblyInstructionBuilder::new()
            .with_command("ADD")
            .with_arg("HL")
            .with_arg(self.0)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::system::Register;

    #[test]
    fn it_disassembles_properly() {
        let op = Add16Operation(WideRegister::AF);
        let instruction = op.disassemble(&Cartridge::default(), 0).unwrap();

        assert_eq!("ADD HL,AF", instruction.to_string());
    }

    #[test]
    fn it_adds_the_lower_byte() {
        let mut state = State::default();
        state.cpu.set16(WideRegister::BC, 0x0010);

        assert_eq!(0x00, state.cpu.get(Register::H));
        assert_eq!(0x00, state.cpu.get(Register::L));

        Add16Operation(WideRegister::BC).act(&mut state).unwrap();

        assert_eq!(0x00, state.cpu.get(Register::H));
        assert_eq!(0x10, state.cpu.get(Register::L));
        assert!(!state.cpu.has_flag(Flags::HALF_CARRY));
        assert!(!state.cpu.has_flag(Flags::CARRY));
    }

    #[test]
    fn it_adds_the_upper_byte() {
        let mut state = State::default();
        state.cpu.set16(WideRegister::BC, 0x0F10);

        assert_eq!(0x00, state.cpu.get(Register::H));
        assert_eq!(0x00, state.cpu.get(Register::L));

        Add16Operation(WideRegister::BC).act(&mut state).unwrap();

        assert_eq!(0x0F, state.cpu.get(Register::H));
        assert_eq!(0x10, state.cpu.get(Register::L));
        assert!(!state.cpu.has_flag(Flags::HALF_CARRY));
        assert!(!state.cpu.has_flag(Flags::CARRY));
    }

    #[test]
    fn it_sets_half_carry() {
        let mut state = State::default();
        state.cpu.set(Register::H, 0b0000_1000);
        state.cpu.set(Register::B, 0b0000_1000);

        assert_eq!(0x08, state.cpu.get(Register::H));
        assert_eq!(0x00, state.cpu.get(Register::L));

        Add16Operation(WideRegister::BC).act(&mut state).unwrap();

        assert_eq!(0x10, state.cpu.get(Register::H));
        assert_eq!(0x00, state.cpu.get(Register::L));
        assert!(state.cpu.has_flag(Flags::HALF_CARRY));
        assert!(!state.cpu.has_flag(Flags::CARRY));
    }

    #[test]
    fn it_sets_carry() {
        let mut state = State::default();
        state.cpu.set16(WideRegister::HL, 0xFFFF);
        state.cpu.set16(WideRegister::BC, 0x0001);

        assert_eq!(0xFF, state.cpu.get(Register::H));
        assert_eq!(0xFF, state.cpu.get(Register::L));

        Add16Operation(WideRegister::BC).act(&mut state).unwrap();

        assert_eq!(0x00, state.cpu.get(Register::H));
        assert_eq!(0x00, state.cpu.get(Register::L));
        assert!(state.cpu.has_flag(Flags::HALF_CARRY));
        assert!(state.cpu.has_flag(Flags::CARRY));
    }

    #[test]
    fn it_clears_the_subtraction_flag() {
        let mut state = State::default();
        state.cpu.set_flag(Flags::SUBTRACTION);

        Add16Operation(WideRegister::BC).act(&mut state).unwrap();

        assert!(!state.cpu.has_flag(Flags::SUBTRACTION));
    }
}
