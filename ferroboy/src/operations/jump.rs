use crate::helpers::word_to_u16;
use crate::operations::increment_clock::IncrementClockOperation;
use crate::operations::operation::ChainableOperation;
use crate::state::State;
use crate::system::{Flags, Register};

#[derive(Debug)]
pub enum JumpRelativeFlag {
    Nop,
    Zero,
    NotZero,
    Carry,
    NotCarry,
}

#[derive(Debug)]
pub struct JumpRelativeOperation(pub JumpRelativeFlag);

impl ChainableOperation for JumpRelativeOperation {
    type Output = IncrementClockOperation;

    fn act(&self, state: &mut State) -> Result<Self::Output, String> {
        let offset = state.read_byte()? as u16;
        let program_counter = state.cpu.get16(Register::PC)?;

        match self.0 {
            JumpRelativeFlag::Nop => {
                state.jump(program_counter + offset)?;
                Ok(IncrementClockOperation(12))
            }
            JumpRelativeFlag::Carry => {
                if state.cpu.has_flag(Flags::CARRY) {
                    state.jump(program_counter + offset)?;
                    return Ok(IncrementClockOperation(12));
                }

                Ok(IncrementClockOperation(8))
            }
            JumpRelativeFlag::NotCarry => {
                if !state.cpu.has_flag(Flags::CARRY) {
                    state.jump(program_counter + offset)?;
                    return Ok(IncrementClockOperation(12));
                }

                Ok(IncrementClockOperation(8))
            }
            JumpRelativeFlag::Zero => {
                if state.cpu.has_flag(Flags::ZERO) {
                    state.jump(program_counter + offset)?;
                    return Ok(IncrementClockOperation(12));
                }

                Ok(IncrementClockOperation(8))
            }
            JumpRelativeFlag::NotZero => {
                if !state.cpu.has_flag(Flags::ZERO) {
                    state.jump(program_counter + offset)?;
                    return Ok(IncrementClockOperation(12));
                }

                Ok(IncrementClockOperation(8))
            }
        }
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

impl ChainableOperation for JumpPositionOperation {
    type Output = IncrementClockOperation;

    fn act(&self, state: &mut State) -> Result<Self::Output, String> {
        if JumpPositionFlags::Register.eq(&self.0) {
            let address = state.cpu.get16(Register::HL)?;
            state.jump(address)?;

            return Ok(IncrementClockOperation(4));
        }

        let address = word_to_u16(state.read_word()?);

        match self.0 {
            JumpPositionFlags::Nop => {
                state.jump(address)?;
                Ok(IncrementClockOperation(16))
            }
            JumpPositionFlags::Zero => {
                if state.cpu.has_flag(Flags::ZERO) {
                    state.jump(address)?;
                    return Ok(IncrementClockOperation(16));
                }

                Ok(IncrementClockOperation(12))
            }
            JumpPositionFlags::NotZero => {
                if !state.cpu.has_flag(Flags::ZERO) {
                    state.jump(address)?;
                    return Ok(IncrementClockOperation(16));
                }

                Ok(IncrementClockOperation(12))
            }
            JumpPositionFlags::Carry => {
                if state.cpu.has_flag(Flags::CARRY) {
                    state.jump(address)?;
                    return Ok(IncrementClockOperation(16));
                }

                Ok(IncrementClockOperation(12))
            }
            JumpPositionFlags::NotCarry => {
                if !state.cpu.has_flag(Flags::CARRY) {
                    state.jump(address)?;
                    return Ok(IncrementClockOperation(16));
                }

                Ok(IncrementClockOperation(12))
            }
            JumpPositionFlags::Register => unreachable!(),
        }
    }
}
