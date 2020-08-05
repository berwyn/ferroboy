use crate::system::Register;

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
    AF,
    /// Register BC uses Register B as the high byte and register
    /// C as the low byte, creating a pseudo-16bit register.
    BC,
    /// Register DE uses Register D as the high byte and register
    /// E as the low byte, creating a pseudo-16bit register.
    DE,
    /// Register HL uses Register H as the high byte and register
    /// L as the low byte, creating a pseudo-16bit register.
    HL,
    /// The stack pointer
    SP,
    /// The program counter
    PC,
}

impl core::convert::TryFrom<WideRegister> for (Register, Register) {
    type Error = String;

    fn try_from(value: WideRegister) -> core::result::Result<Self, Self::Error> {
        let pair = match value {
            WideRegister::AF => (Register::A, Register::F),
            WideRegister::BC => (Register::B, Register::C),
            WideRegister::DE => (Register::D, Register::E),
            WideRegister::HL => (Register::H, Register::L),
            _ => return Err("SP and PC cannot be represented as 8-bit registers".into()),
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
                WideRegister::AF => "AF",
                WideRegister::BC => "BC",
                WideRegister::DE => "DE",
                WideRegister::HL => "HL",
                WideRegister::SP => "SP",
                WideRegister::PC => "PC",
            }
        )
    }
}
