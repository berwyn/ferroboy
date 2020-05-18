use core::convert::TryInto;

use crate::assembly::{AssemblyInstruction, AssemblyInstructionBuilder, Disassemble};
use crate::helpers::word_to_u16;
use crate::operations::Operation;
use crate::state::State;
use crate::system::{Register, WideRegister};

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

impl Operation for Load8ImmediateOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        let value = state.read_byte()?;
        state.cpu.set(self.0, value);
        state.cpu.increment_clock(8);
        Ok(())
    }
}

impl Disassemble for Load8ImmediateOperation {
    fn disassemble(&self, state: &mut State) -> crate::Result<AssemblyInstruction> {
        let immediate = word_to_u16(state.read_word()?);

        AssemblyInstructionBuilder::new()
            .with_command("LD")
            .with_arg(self.0)
            .with_arg(format!("${:X}", immediate))
            .build()
    }
}

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
#[derive(Clone, Copy, Debug)]
pub struct Load8RegisterCopyOperation(pub Register, pub Register);

impl Operation for Load8RegisterCopyOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        let value = state.cpu.get(self.1);
        state.cpu.set(self.0, value);
        state.cpu.increment_clock(4);
        Ok(())
    }
}

impl Disassemble for Load8RegisterCopyOperation {
    fn disassemble(&self, _: &mut State) -> crate::Result<AssemblyInstruction> {
        AssemblyInstructionBuilder::new()
            .with_command("LD")
            .with_arg(self.0)
            .with_arg(self.1)
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
#[derive(Clone, Copy, Debug)]
pub struct Load8FromMemoryOperation(pub Register, pub WideRegister);

impl Operation for Load8FromMemoryOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        let (high, low) = self.1.try_into()?;

        let address_high = state.cpu.get(high);
        let address_low = state.cpu.get(low);
        let address = word_to_u16((address_high, address_low));
        let value = state.mmu[address];

        state.cpu.set(self.0, value);
        state.cpu.increment_clock(8);
        Ok(())
    }
}

impl Disassemble for Load8FromMemoryOperation {
    fn disassemble(&self, _: &mut State) -> crate::Result<AssemblyInstruction> {
        AssemblyInstructionBuilder::new()
            .with_command("LD")
            .with_arg(self.0)
            .with_arg(format!("({})", self.1))
            .build()
    }
}

// TODO: LD (HL),d8 (0x36)
// TODO: LD (a16),SP (0x08)
// TODO: LD (HL+,A) / LD (HL-),A
// ? What are the plus and minus?

#[derive(Clone, Copy, Debug)]
pub enum Load8RegisterToMemoryTarget {
    // ? Should this lose it's argument? Can only be C
    Register(Register),
    WideRegister(WideRegister),
}

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
#[derive(Clone, Copy, Debug)]
pub struct Load8RegisterToMemoryOperation(pub Load8RegisterToMemoryTarget, pub Register);

impl Operation for Load8RegisterToMemoryOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        let address = match self.0 {
            Load8RegisterToMemoryTarget::WideRegister(r) => state.cpu.get16(r),
            Load8RegisterToMemoryTarget::Register(r) => u16::from(state.cpu.get(r)),
        };

        let value = state.cpu.get(self.1);
        state.mmu[address] = value;

        state.cpu.increment_clock(8);

        Ok(())
    }
}

impl Disassemble for Load8RegisterToMemoryOperation {
    fn disassemble(&self, _: &mut State) -> crate::Result<AssemblyInstruction> {
        AssemblyInstructionBuilder::new()
            .with_command("LD")
            .with_arg(format!(
                "({})",
                match self.0 {
                    Load8RegisterToMemoryTarget::WideRegister(r) => r.to_string(),
                    Load8RegisterToMemoryTarget::Register(r) => r.to_string(),
                }
            ))
            .with_arg(self.1)
            .build()
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

        assert_eq!(0x00, state.cpu.get(Register::B));

        op.act(&mut state).unwrap();

        assert_eq!(0xFE, state.cpu.get(Register::B));
    }

    #[test]
    fn it_loads_a_value_from_one_register_to_another() {
        let mut state = State::default();
        let op = Load8RegisterCopyOperation(Register::B, Register::A);

        state.cpu.set(Register::A, 0xFE);

        assert_eq!(0x00, state.cpu.get(Register::B));
        assert_eq!(0xFE, state.cpu.get(Register::A));

        op.act(&mut state).unwrap();

        assert_eq!(0xFE, state.cpu.get(Register::B));
        assert_eq!(0xFE, state.cpu.get(Register::A));
    }

    #[test]
    fn it_loads_a_value_from_memory_to_register() {
        let mut state = State::default();
        let op = Load8FromMemoryOperation(Register::B, WideRegister::HL);

        state.mmu.mutate(|mmu| mmu[0x5E50] = 0xFE);
        state.cpu.set(Register::H, 0x5E);
        state.cpu.set(Register::L, 0x50);

        assert_eq!(0x00, state.cpu.get(Register::B));

        op.act(&mut state).unwrap();

        assert_eq!(0xFE, state.cpu.get(Register::B));
    }

    #[test]
    fn it_writes_a_register_into_memory() {
        let mut state = State::default();
        let op = Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::WideRegister(WideRegister::PC),
            Register::A,
        );

        state.cpu.set16(WideRegister::PC, 0x5E50);
        state.cpu.set(Register::A, 0xBE);

        assert_eq!(0x00, state.mmu[0x5E50]);

        op.act(&mut state).unwrap();

        assert_eq!(0xBE, state.mmu[0x5E50]);
    }

    #[test]
    fn it_disassembles_immediate_to_register() {
        let mut state = State::default();
        state.mmu.mutate(|mmu| {
            mmu[0x00] = 0xBE;
            mmu[0x01] = 0xEF;
        });

        let op = Load8ImmediateOperation(Register::A);

        assert_eq!(
            "LD A,$BEEF",
            op.disassemble(&mut state).unwrap().to_string()
        );
    }

    #[test]
    fn it_dissassembles_register_to_register() {
        let op = Load8RegisterCopyOperation(Register::A, Register::B);
        let instruction = op.disassemble(&mut State::default()).unwrap();

        assert_eq!("LD A,B", instruction.to_string());
    }

    #[test]
    fn it_disassembles_memory_to_register() {
        let op = Load8FromMemoryOperation(Register::A, WideRegister::BC);
        let instruction = op.disassemble(&mut State::default()).unwrap();

        assert_eq!("LD A,(BC)", instruction.to_string());
    }

    #[test]
    fn it_dissassembles_register_to_memory() {
        let op = Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::WideRegister(WideRegister::HL),
            Register::A,
        );
        let instruction: AssemblyInstruction = op.disassemble(&mut State::default()).unwrap();

        assert_eq!("LD (HL),A", instruction.to_string());
    }
}
