use crate::assembly::{AssemblyInstruction, AssemblyInstructionBuilder, Disassemble};
use crate::helpers::word_to_u16;
use crate::operations::Operation;
use crate::state::State;
use crate::system::{Flags, WideRegister};

/// Indicates what condition should trigger a relative jump command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JumpRelativeFlag {
    /// The jump should always occur.
    Nop,
    /// The jump should only occur if the zero flag is set.
    Zero,
    /// The jump should only occur if the zero flag is unset.
    NotZero,
    /// The jump should only occur if the carry flag is set.
    Carry,
    /// The jump should only occur if the carry flag is unset.
    NotCarry,
}

impl std::fmt::Display for JumpRelativeFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Nop => "<nop>",
                Self::Zero => "Z",
                Self::NotZero => "NZ",
                Self::Carry => "C",
                Self::NotCarry => "NC",
            }
        )
    }
}

/// Reads a signed 8-bit integer and adds it to the program counter if
/// the conditions of the flag are met.
///
/// # Opcode Reference
/// ## Assembly Definition
/// ```a
/// ; Nop
/// JR $78
/// ; Zero
/// JR Z,$78
/// ; NotZero
/// JR NZ,$78
/// ; Carry
/// JR C,$78
/// ; NotCarry
/// JR NC,$78
/// ```
/// ## Runtime
/// | Metric | Size |
/// |:-------|:-----|
/// | Length | 2 |
/// | Cycles | (see below) |
///
/// Cycle count for JR depends on two factors:
/// - Does it branch?
/// - Is the branch condition met?
///
/// In cases like `Nop`, where there's no branch condition, and cases
/// where the branch condition is met `JR` consumes 12 cycles. In all
/// other cases, `JR` consumes 8 cycles.
///
/// | Condition | Condition met? | Cycles |
/// |:----------|:---------------|:-------|
/// | `Nop` | | 12 |
/// | `Zero` | ❌ | 8 |
/// | `Zero` | ✅ | 12 |
/// | `NotZero` | ❌ | 8 |
/// | `NotZero` | ✅ | 12 |
/// | `Carry` | ❌ | 8 |
/// | `Carry` | ✅ | 12 |
/// | `NotCarry` | ❌ | 8 |
/// | `NotCarry` | ✅ | 12 |
/// ## Flags
/// | Flag | Value |
/// |:-----|:------|
/// | Zero | Not Affected |
/// | Subtraction | Not Affected |
/// | Half-Cary | Not Affected |
/// | Carry | Not Affected |
///
/// # Examples
/// ```rs
/// let operation = JumpRelativeOperation(JumpRelativeFlag::Zero);
/// operation.act(&mut state).unwrap();
/// ```
///
/// # Errors
/// - This should only error if the program counter points outside valid memory
#[derive(Debug)]
pub struct JumpRelativeOperation(pub JumpRelativeFlag);

impl Operation for JumpRelativeOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        let offset = state.read_byte()? as u16;
        let program_counter = state.cpu.get16(WideRegister::PC)?;

        match self.0 {
            JumpRelativeFlag::Nop => {
                state.jump(program_counter + offset)?;
                state.cpu.increment_clock(12);
                Ok(())
            }
            JumpRelativeFlag::Carry => {
                if state.cpu.has_flag(Flags::CARRY) {
                    state.jump(program_counter + offset)?;
                    state.cpu.increment_clock(12);
                } else {
                    state.cpu.increment_clock(8);
                }

                Ok(())
            }
            JumpRelativeFlag::NotCarry => {
                if !state.cpu.has_flag(Flags::CARRY) {
                    state.jump(program_counter + offset)?;
                    state.cpu.increment_clock(12);
                } else {
                    state.cpu.increment_clock(8);
                }

                Ok(())
            }
            JumpRelativeFlag::Zero => {
                if state.cpu.has_flag(Flags::ZERO) {
                    state.jump(program_counter + offset)?;
                    state.cpu.increment_clock(12);
                } else {
                    state.cpu.increment_clock(8);
                }

                Ok(())
            }
            JumpRelativeFlag::NotZero => {
                if !state.cpu.has_flag(Flags::ZERO) {
                    state.jump(program_counter + offset)?;
                    state.cpu.increment_clock(12);
                } else {
                    state.cpu.increment_clock(8);
                }

                Ok(())
            }
        }
    }
}

impl Disassemble for JumpRelativeOperation {
    fn disassemble(&self, state: &mut State) -> crate::Result<AssemblyInstruction> {
        let immediate = word_to_u16(state.read_word()?);

        let mut builder = AssemblyInstructionBuilder::new().with_command("JR");

        if !JumpRelativeFlag::Nop.eq(&self.0) {
            builder = builder.with_arg(self.0.clone());
        }

        builder.with_arg(format!("${:X}", immediate)).build()
    }
}

/// Indicates what conditions should trigger a jump position command
#[derive(Clone, Debug, PartialEq)]
pub enum JumpPositionFlags {
    /// The jump should always occur.
    Nop,
    /// The jump should only occur if the zero flag is set.
    Zero,
    /// The jump should only occur if the zero flag is unset.
    NotZero,
    /// The jump should only occur if the carry flag is set.
    Carry,
    /// The jump should only occur if the carry flag is unset.
    NotCarry,
    /// The jump should always occur, and read the address from HL.
    Register,
}

impl std::fmt::Display for JumpPositionFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Nop => "<nop>",
                Self::Zero => "Z",
                Self::NotZero => "NZ",
                Self::Carry => "C",
                Self::NotCarry => "NC",
                Self::Register => "<register>",
            }
        )
    }
}

/// Reads a signed 8-bit integer and adds it to the program counter if
/// the conditions of the flag are met.
///
/// # Opcode Reference
/// ## Assembly Definition
/// ```a
/// ; Nop
/// JP $78
/// ; Zero
/// JP Z,$78
/// ; NotZero
/// JP NZ,$78
/// ; Carry
/// JP C,$78
/// ; NotCarry
/// JP NC,$78
/// ; Register
/// JP (HL)
/// ```
/// ## Runtime
/// | Metric | Size        |
/// |:-------|:------------|
/// | Length | (see below) |
/// | Cycles | (see below) |
///
/// When it comes to the length of the `JP` instruction, most occurances
/// are 3 bytes, being the opcode and the address. There is one special case,
/// however, in that `JP (HL)` is only the opcode, and therefore 1 byte.
///
/// | Condition | Size |
/// |:----------|:-----|
/// | `Nop`     | 3    |
/// | `Z`/`NZ`  | 3    |
/// | `C`/`NC`  | 3    |
/// | `Register`| 1    |
///
/// In regards to cycle count, this once again varies depending on the conditions.
/// `JP` instructions that need to read the immediate 16-bit address will be
/// 16 cycles on a successful jump, or 12 cycles where no jump is made. `JP (HL)`
/// is once again a special case, and only consumes 4 cycles.
///
/// | Condition  | Condition met? | Cycles |
/// |:-----------|:---------------|:-------|
/// | `Nop`      |                | 16     |
/// | `Zero`     | ❌             | 12     |
/// | `Zero`     | ✅             | 16     |
/// | `NotZero`  | ❌             | 12     |
/// | `NotZero`  | ✅             | 16     |
/// | `Carry`    | ❌             | 12     |
/// | `Carry`    | ✅             | 16     |
/// | `NotCarry` | ❌             | 12     |
/// | `NotCarry` | ✅             | 16     |
/// | `Register` |                | 4      |
/// ## Flags
/// | Flag          | Value         |
/// |:--------------|:--------------|
/// | Zero          | Not Affected  |
/// | Subtraction   | Not Affected  |
/// | Half-Cary     | Not Affected  |
/// | Carry         | Not Affected  |
///
/// # Examples
/// ```rs
/// let operation = JumpPositionOperation(JumpRelativeFlag::Zero);
/// operation.act(&mut state).unwrap();
/// ```
///
/// # Errors
/// - This should only error if the program counter points outside valid memory
#[derive(Debug)]
pub struct JumpPositionOperation(pub JumpPositionFlags);

impl Operation for JumpPositionOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        if JumpPositionFlags::Register.eq(&self.0) {
            let address = state.cpu.get16(WideRegister::HL)?;
            state.jump(address)?;
            state.cpu.increment_clock(4);

            return Ok(());
        }

        let address = word_to_u16(state.read_word()?);

        match self.0 {
            JumpPositionFlags::Nop => {
                state.jump(address)?;
                state.cpu.increment_clock(16);
                Ok(())
            }
            JumpPositionFlags::Zero => {
                if state.cpu.has_flag(Flags::ZERO) {
                    state.jump(address)?;
                    state.cpu.increment_clock(16);
                } else {
                    state.cpu.increment_clock(12);
                }

                Ok(())
            }
            JumpPositionFlags::NotZero => {
                if !state.cpu.has_flag(Flags::ZERO) {
                    state.jump(address)?;
                    state.cpu.increment_clock(16);
                } else {
                    state.cpu.increment_clock(12);
                }

                Ok(())
            }
            JumpPositionFlags::Carry => {
                if state.cpu.has_flag(Flags::CARRY) {
                    state.jump(address)?;
                    state.cpu.increment_clock(16);
                } else {
                    state.cpu.increment_clock(12);
                }

                Ok(())
            }
            JumpPositionFlags::NotCarry => {
                if !state.cpu.has_flag(Flags::CARRY) {
                    state.jump(address)?;
                    state.cpu.increment_clock(16);
                } else {
                    state.cpu.increment_clock(12);
                }

                Ok(())
            }
            JumpPositionFlags::Register => unreachable!(),
        }
    }
}

impl Disassemble for JumpPositionOperation {
    fn disassemble(&self, state: &mut State) -> crate::Result<AssemblyInstruction> {
        let immediate = word_to_u16(state.read_word()?);

        let mut builder = AssemblyInstructionBuilder::new().with_command("JP");

        match self.0 {
            JumpPositionFlags::Nop => {
                builder = builder.with_arg(format!("${:X}", immediate));
            }
            JumpPositionFlags::Register => {
                builder = builder.with_arg("(HL)");
            }
            _ => {
                builder = builder
                    .with_arg(self.0.clone())
                    .with_arg(format!("${:X}", immediate))
            }
        }

        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jump_relative_disassembles_correctly() {
        let mut state = State::default();
        state.mmu.mutate(|mmu| {
            mmu[0x00] = 0xFF;
            mmu[0x01] = 0xFF;
            mmu[0x02] = 0xBE;
            mmu[0x03] = 0xEF;
        });

        let nop = JumpRelativeOperation(JumpRelativeFlag::Nop);
        let nop_instruction: AssemblyInstruction = nop.disassemble(&mut state).unwrap();
        assert_eq!("JR $FFFF", nop_instruction.to_string());

        let zero = JumpRelativeOperation(JumpRelativeFlag::Zero);
        let zero_instruction: AssemblyInstruction = zero.disassemble(&mut state).unwrap();
        assert_eq!("JR Z,$BEEF", zero_instruction.to_string());
    }

    #[test]
    fn jump_position_disassembles_correctly() -> crate::Result<()> {
        let mut state = State::default();
        state.mmu.mutate(|mmu| {
            mmu[0x00] = 0xFF;
            mmu[0x01] = 0xFF;
            mmu[0x02] = 0xDE;
            mmu[0x03] = 0xAD;
        });

        let nop = JumpPositionOperation(JumpPositionFlags::Nop);
        assert_eq!("JP $FFFF", nop.disassemble(&mut state)?.to_string());

        let zero = JumpPositionOperation(JumpPositionFlags::Zero);
        assert_eq!("JP Z,$DEAD", zero.disassemble(&mut state)?.to_string());

        let register = JumpPositionOperation(JumpPositionFlags::Register);
        assert_eq!("JP (HL)", register.disassemble(&mut state)?.to_string());

        Ok(())
    }
}
