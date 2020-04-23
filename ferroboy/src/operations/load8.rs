use crate::assembly::{AssemblyInstruction, AssemblyInstructionBuilder};
use crate::helpers::word_to_u16;
use crate::operations::Operation;
use crate::state::State;
use crate::system::Register;

// ? Should this be split up into separate files?

/// Loads an immediate 8-bit value into a register.
///
/// # Opcode Reference
/// ## Assembly definition
/// ```a
/// LD A,#
/// ```
///
/// ## Runtime
/// | Metric | Size |
/// |:-------|:-----|
/// | Length | 2    |
/// | Cycles | 8    |
///
/// ## Flags
/// | Flag          | Value        |
/// |:--------------|:-------------|
/// | Zero          | Not Affected |
/// | Subtraction   | Not Affected |
/// | Half-Carry    | Not Affected |
/// | Carry         | Not Affected |
///
/// # Examples
/// ```rs
/// Load8ImmediateOperation(Register::A)
/// ```
///
/// # Errors
/// - The operation may fail if provided a 16-bit register
#[derive(Debug)]
pub struct Load8ImmediateOperation(pub Register);
// FIXME: Register refactor

impl Operation for Load8ImmediateOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        let value = state.read_byte()?;
        state.cpu.set(self.0, value).map(|_| ())?;
        state.cpu.increment_clock(8);
        Ok(())
    }
}

// TODO: Implement Disassemble

/// Copy the value of one register to another.
///
/// # Opcode Reference
/// ## Assembly definition
/// ```a
/// LD A,B
/// ```
///
/// ## Runtime
/// | Metric | Size |
/// |:-------|:-----|
/// | Length | 1    |
/// | Cycles | 4    |
///
/// ## Flags
/// | Flag          | Value        |
/// |:--------------|:-------------|
/// | Zero          | Not Affected |
/// | Subtraction   | Not Affected |
/// | Half-Carry    | Not Affected |
/// | Carry         | Not Affected |
///
/// # Examples
/// ```rs
/// Load8RegisterCopyOperation(Register::A, Register::B)
/// ```
///
/// # Errors
/// - The operation may fail if provided a 16-bit register
#[derive(Debug)]
pub struct Load8RegisterCopyOperation(pub Register, pub Register);
// FIXME: Register refactor

impl Operation for Load8RegisterCopyOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        let value = state.cpu.get(self.1)?;
        state.cpu.set(self.0, value).map(|_| ())?;
        state.cpu.increment_clock(4);
        Ok(())
    }
}

impl core::convert::TryFrom<Load8RegisterCopyOperation> for AssemblyInstruction {
    type Error = String;

    fn try_from(value: Load8RegisterCopyOperation) -> Result<Self, Self::Error> {
        AssemblyInstructionBuilder::new()
            .with_command("LD")
            .with_arg(value.0)
            .with_arg(value.1)
            .build()
    }
}

// FIXME: This doesn't work for LD A,(C), LD A,(a8) or LD A,(a16)
// ? Maybe a flag enum? Load8MemorySource with values Register, WideRegister, Immediate, WideImmediate?
// FIXME: Metrics here are based as-implemented, but should be updated when impl is correct
// e.g. LD A,(a16) is 3 bytes, 16 cycles vs LD A,(HL) at 1 byte 8 cycles
/// Load an 8-bit value from the address stored in a 16-bit register.
///
/// # Opcode Reference
/// ## Assembly definition
/// ```a
/// LD A,(HL)
/// ```
///
/// ## Runtime
/// | Metric | Size |
/// |:-------|:-----|
/// | Length | 1    |
/// | Cycles | 8    |
///
/// ## Flags
/// | Flag          | Value        |
/// |:--------------|:-------------|
/// | Zero          | Not Affected |
/// | Subtraction   | Not Affected |
/// | Half-Carry    | Not Affected |
/// | Carry         | Not Affected |
///
/// # Examples
/// ```rs
/// Load8RegisterCopyOperation(Register::A, Register::B)
/// ```
///
/// # Errors
/// - The operation may fail if the first argument is a 16-bit register
#[derive(Debug)]
pub struct Load8FromMemoryOperation(pub Register, pub Register);

impl Operation for Load8FromMemoryOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        let (high, low) = Register::to_8bit_pair(self.1)?;

        let address_high = state.cpu.get(high)?;
        let address_low = state.cpu.get(low)?;
        let address = word_to_u16((address_high, address_low));
        let value = state.mmu[address];

        state.cpu.set(self.0, value).map(|_| ())?;
        state.cpu.increment_clock(8);
        Ok(())
    }
}

// TODO: LD (HL),d8 (0x36)
// TODO: LD (a16),SP (0x08)
// TODO: LD (HL+,A) / LD (HL-),A
// ? What are the plus and minus?

// FIXME: Metrics here are based on implementation and should be fixed
// FIXME: Doesn't support LD (a16),A
/// Copies a value from a register to the address held in a register.
///
/// # Opcode Reference
/// ## Assembly definition
/// ```a
/// LD (HL),A
/// ```
///
/// ## Runtime
/// | Metric | Size |
/// |:-------|:-----|
/// | Length | 1*   |
/// | Cycles | 8*   |
///
/// `*`: `LD (C),A` (0xE2) is 2 bytes and 8 cycles.
///
/// ## Flags
/// | Flag          | Value        |
/// |:--------------|:-------------|
/// | Zero          | Not Affected |
/// | Subtraction   | Not Affected |
/// | Half-Carry    | Not Affected |
/// | Carry         | Not Affected |
///
/// # Examples
/// ```rs
/// Load8RegisterToMemoryOperation(Register::HL, Register::A)
/// ```
///
/// # Errors
/// - The operation may fail if a 16-bit register is provided as the source
#[derive(Debug)]
pub struct Load8RegisterToMemoryOperation(pub Register, pub Register);

impl Operation for Load8RegisterToMemoryOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        let address = match self.0 {
            Register::PC | Register::BC | Register::DE | Register::HL => state.cpu.get16(self.0)?,
            Register::C => u16::from(state.cpu.get(self.0)?),
            _ => return Err("Invalid register provided".into()),
        };

        let value = state.cpu.get(self.1)?;
        state.mmu[address] = value;

        state.cpu.increment_clock(8);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_loads_an_immediate_into_the_register() {
        let mut state = State::default();
        state.mmu.mutate(|m| m[0x00] = 0xFE);

        let op = Load8ImmediateOperation(Register::B);

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0xFE, state.cpu.get(Register::B).unwrap());
    }

    #[test]
    fn it_loads_a_value_from_one_register_to_another() {
        let mut state = State::default();
        let op = Load8RegisterCopyOperation(Register::B, Register::A);

        state.cpu.set(Register::A, 0xFE).unwrap();

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());
        assert_eq!(0xFE, state.cpu.get(Register::A).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0xFE, state.cpu.get(Register::B).unwrap());
        assert_eq!(0xFE, state.cpu.get(Register::A).unwrap());
    }

    #[test]
    fn it_loads_a_value_from_memory_to_register() {
        let mut state = State::default();
        let op = Load8FromMemoryOperation(Register::B, Register::HL);

        state.mmu.mutate(|mmu| mmu[0x5E50] = 0xFE);
        state.cpu.set(Register::H, 0x5E).unwrap();
        state.cpu.set(Register::L, 0x50).unwrap();

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0xFE, state.cpu.get(Register::B).unwrap());
    }

    #[test]
    fn it_writes_a_register_into_memory() {
        let mut state = State::default();
        let op = Load8RegisterToMemoryOperation(Register::PC, Register::A);

        state.cpu.set16(Register::PC, 0x5E50).unwrap();
        state.cpu.set(Register::A, 0xBE).unwrap();

        assert_eq!(0x00, state.mmu[0x5E50]);

        op.act(&mut state).unwrap();

        assert_eq!(0xBE, state.mmu[0x5E50]);
    }
}
