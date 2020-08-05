use crate::{
    assembly::{AssemblyInstruction, AssemblyInstructionBuilder, Disassemble},
    helpers::word_to_u16,
    operations::Operation,
    state::State,
    system::{Cartridge, Flags, WideRegister},
};

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
///
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
            let address = state.cpu.get16(WideRegister::HL);
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
    fn disassemble(
        &self,
        cartridge: &Cartridge,
        offset: usize,
    ) -> crate::Result<AssemblyInstruction> {
        let word = (cartridge.data[offset + 1], cartridge.data[offset + 2]);
        let immediate = word_to_u16(word);

        let mut builder = AssemblyInstructionBuilder::new().with_command("JP");

        match self.0 {
            JumpPositionFlags::Nop => {
                builder = builder.with_arg(format!("${:X}", immediate)).with_size(3);
            }
            JumpPositionFlags::Register => {
                builder = builder.with_arg("(HL)");
            }
            _ => {
                builder = builder
                    .with_arg(self.0.clone())
                    .with_arg(format!("${:X}", immediate))
                    .with_size(3)
            }
        }

        builder.build()
    }

    fn describe(&self) -> crate::Result<AssemblyInstruction> {
        let mut builder = AssemblyInstructionBuilder::new().with_command("JP");

        match self.0 {
            JumpPositionFlags::Nop => builder = builder.with_arg("aa").with_size(3),
            JumpPositionFlags::Register => builder = builder.with_arg("(HL)"),
            _ => builder = builder.with_arg(self.0.clone()).with_arg("aa").with_size(3),
        }

        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod disassemble {
        use super::*;

        #[test]
        fn it_disassembles_correctly() -> crate::Result<()> {
            let mut cartridge = Cartridge::default();
            cartridge.data = vec![0x00, 0xFF, 0xFF, 0xDE, 0xAD];

            let nop = JumpPositionOperation(JumpPositionFlags::Nop);
            assert_eq!("JP $FFFF", nop.disassemble(&cartridge, 0)?.to_string());

            let zero = JumpPositionOperation(JumpPositionFlags::Zero);
            assert_eq!("JP Z,$DEAD", zero.disassemble(&cartridge, 2)?.to_string());

            let register = JumpPositionOperation(JumpPositionFlags::Register);
            assert_eq!("JP (HL)", register.disassemble(&cartridge, 0)?.to_string());

            Ok(())
        }
    }
}
