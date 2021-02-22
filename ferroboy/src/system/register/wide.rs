use crate::{error::OperationError, system::Register};

/// 16-bit registers.
///
/// DMG-01 has a few 16-bit registers, composed of the pseudo-16-bit
/// registers that use 8-bit registers to store their high- and low-nybbles
/// as well as the stack pointer and program counter which are properly
/// 16-bit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WideRegister {
    /// Register AF uses the Accumulator as the high byte and
    /// the flags as the low byte, creating a pseudo-16bit register.
    Af,
    /// Register BC uses Register B as the high byte and register
    /// C as the low byte, creating a pseudo-16bit register.
    Bc,
    /// Register DE uses Register D as the high byte and register
    /// E as the low byte, creating a pseudo-16bit register.
    De,
    /// Register HL uses Register H as the high byte and register
    /// L as the low byte, creating a pseudo-16bit register.
    Hl,
    /// The stack pointer
    Sp,
    /// The program counter
    Pc,
}

impl core::convert::TryFrom<WideRegister> for (Register, Register) {
    type Error = crate::Error;

    fn try_from(value: WideRegister) -> core::result::Result<Self, Self::Error> {
        let pair = match value {
            WideRegister::Af => (Register::A, Register::F),
            WideRegister::Bc => (Register::B, Register::C),
            WideRegister::De => (Register::D, Register::E),
            WideRegister::Hl => (Register::H, Register::L),
            _ => return Err(OperationError::InvalidWideRegister(value).into()),
        };

        Ok(pair)
    }
}

impl std::fmt::Display for WideRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WideRegister::Af => "AF",
                WideRegister::Bc => "BC",
                WideRegister::De => "DE",
                WideRegister::Hl => "HL",
                WideRegister::Sp => "SP",
                WideRegister::Pc => "PC",
            }
        )
    }
}
