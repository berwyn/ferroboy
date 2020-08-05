use crate::{
    assembly::{AssemblyInstruction, AssemblyInstructionBuilder, Disassemble},
    operations::Operation,
    state::State,
    system::{Cartridge, Flags, WideRegister},
};

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
/// | Metric | Size        |
/// |:-------|:------------|
/// | Length | 2           |
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
/// | Condition  | Condition met? | Cycles |
/// |:-----------|:---------------|:-------|
/// | `Nop`      |                | 12     |
/// | `Zero`     | ❌             | 8      |
/// | `Zero`     | ✅             | 12     |
/// | `NotZero`  | ❌             | 8      |
/// | `NotZero`  | ✅             | 12     |
/// | `Carry`    | ❌             | 8      |
/// | `Carry`    | ✅             | 12     |
/// | `NotCarry` | ❌             | 8      |
/// | `NotCarry` | ✅             | 12     |
///
/// ## Flags
/// | Flag        | Value        |
/// |:------------|:-------------|
/// | Zero        | Not Affected |
/// | Subtraction | Not Affected |
/// | Half-Cary   | Not Affected |
/// | Carry       | Not Affected |
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
        let program_counter = state.cpu.get16(WideRegister::PC);

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
    fn disassemble(
        &self,
        cartridge: &Cartridge,
        offset: usize,
    ) -> crate::Result<AssemblyInstruction> {
        let immediate = cartridge.data[offset + 1] as i8;

        let mut builder = AssemblyInstructionBuilder::new().with_command("JR");

        if !JumpRelativeFlag::Nop.eq(&self.0) {
            builder = builder.with_arg(self.0.clone());
        }

        builder
            .with_arg(format!("${:X}", immediate))
            .with_size(2)
            .build()
    }

    fn describe(&self) -> crate::Result<AssemblyInstruction> {
        let mut builder = AssemblyInstructionBuilder::new().with_command("JR");

        if !JumpRelativeFlag::Nop.eq(&self.0) {
            builder = builder.with_arg(self.0.clone());
        }

        builder.with_arg("r").with_size(2).build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod disassemble {
        use super::*;

        #[test]
        fn it_disassembles_correctly() {
            let mut cartridge = Cartridge::default();
            cartridge.data = vec![0x00, 0xFF, 0xBE];

            let nop = JumpRelativeOperation(JumpRelativeFlag::Nop);
            let nop_instruction: AssemblyInstruction = nop.disassemble(&cartridge, 0).unwrap();
            assert_eq!("JR $FF", nop_instruction.to_string());

            let zero = JumpRelativeOperation(JumpRelativeFlag::Zero);
            let zero_instruction: AssemblyInstruction = zero.disassemble(&cartridge, 1).unwrap();
            assert_eq!("JR Z,$BE", zero_instruction.to_string());
        }
    }
}
