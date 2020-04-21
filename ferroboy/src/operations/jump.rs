use crate::helpers::word_to_u16;
use crate::operations::Operation;
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
