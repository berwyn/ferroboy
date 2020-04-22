use crate::assembly::{AssemblyInstruction, AssemblyInstructionBuilder};
use crate::helpers::word_to_u16;
use crate::operations::Operation;
use crate::state::State;
use crate::system::{Flags, Register};

/// Indicates what condition should trigger a relative jump command.
#[derive(Debug, PartialEq, Eq)]
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
        let program_counter = state.cpu.get16(Register::PC)?;

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

impl core::convert::TryFrom<JumpRelativeOperation> for AssemblyInstruction {
    type Error = String;

    fn try_from(value: JumpRelativeOperation) -> core::result::Result<Self, Self::Error> {
        let mut builder = AssemblyInstructionBuilder::new().with_command("JR");

        if !value.0.eq(&JumpRelativeFlag::Nop) {
            builder = builder.with_arg(value.0);
        }

        builder.with_arg("#").build()
    }
}

#[derive(Debug, PartialEq)]
pub enum JumpPositionFlags {
    Nop,
    Zero,
    NotZero,
    Carry,
    NotCarry,
    Register,
}

#[derive(Debug)]
pub struct JumpPositionOperation(pub JumpPositionFlags);

impl Operation for JumpPositionOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        if JumpPositionFlags::Register.eq(&self.0) {
            let address = state.cpu.get16(Register::HL)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jump_relative_disassembles_correctly() {
        use core::convert::TryInto;

        let nop = JumpRelativeOperation(JumpRelativeFlag::Nop);
        let nop_instruction: AssemblyInstruction = nop.try_into().unwrap();
        assert_eq!("JR #", nop_instruction.to_string());

        let zero = JumpRelativeOperation(JumpRelativeFlag::Zero);
        let zero_instruction: AssemblyInstruction = zero.try_into().unwrap();
        assert_eq!("JR Z,#", zero_instruction.to_string());
    }
}
