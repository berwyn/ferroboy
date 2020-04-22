use crate::assembly::{AssemblyInstruction, AssemblyInstructionBuilder};
use crate::operations::Operation;
use crate::state::State;
use crate::system::{Flags, Register};

// FIXME: WideRegiser
/// Increments a singular register.
///
/// # Opcode Reference
/// ## Assembly Definition
/// ```a
/// INC BC
/// ```
///
/// ## Runtime
/// | Metric | Size |
/// |:-------|:-----|
/// | Length | 1 |
/// | Cycles | 8 |
///
/// ## Flags
/// | Flag | Value |
/// |:-----|:------|
/// | Zero | Not Affected |
/// | Subtraction | Not Affected |
/// | Half-Carry | Not Affected |
/// | Carry | Not Affected |
///
/// # Examples
/// ```rs
/// let operation = Inc16Operation(Register::BC);
/// operation.act(&mut state).unwrap();
/// ```
///
/// # Errors
/// - The operation may fail if an 8-bit register is provided.
#[derive(Debug)]
pub struct Inc16Operation(pub Register);

impl Operation for Inc16Operation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        match self.0 {
            Register::SP => state.cpu.mutate16(self.0, |old| old + 1).map(|_| ())?,
            _ => {
                let (high, low) = self.0.to_8bit_pair()?;

                let mut lower = u16::from(state.cpu.get(low)?);
                lower += 1;
                state.cpu.set(low, lower as u8).map(|_| ())?;

                if lower / 0xFF > 0 {
                    state.cpu.set_flag(Flags::HALF_CARRY);
                    let mut upper = u16::from(state.cpu.get(high)?);
                    upper += 1;

                    if upper / 0xFF > 0 {
                        state.cpu.set_flag(Flags::CARRY);
                    }

                    state.cpu.set(high, upper as u8).map(|_| ())?;
                }
            }
        };

        state.cpu.increment_clock(8);

        Ok(())
    }
}

impl core::convert::TryFrom<Inc16Operation> for AssemblyInstruction {
    type Error = String;

    fn try_from(value: Inc16Operation) -> Result<AssemblyInstruction, Self::Error> {
        AssemblyInstructionBuilder::new()
            .with_command("INC")
            .with_arg(value.0)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_increments_the_lower_byte() {
        let mut state = State::default();
        let op = Inc16Operation(Register::BC);

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());
        assert_eq!(0x00, state.cpu.get(Register::C).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());
        assert_eq!(0x01, state.cpu.get(Register::C).unwrap());
    }

    #[test]
    fn it_increments_the_upper_byte() {
        let mut state = State::default();
        let op = Inc16Operation(Register::BC);

        state.cpu.set(Register::C, 0xFF).unwrap();

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());
        assert_eq!(0xFF, state.cpu.get(Register::C).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0x01, state.cpu.get(Register::B).unwrap());
        assert_eq!(0x00, state.cpu.get(Register::C).unwrap());
        assert!(state.cpu.has_flag(Flags::HALF_CARRY));
    }

    #[test]
    fn it_wraps_over() {
        let mut state = State::default();
        let op = Inc16Operation(Register::BC);

        state.cpu.set(Register::B, 0xFF).unwrap();
        state.cpu.set(Register::C, 0xFF).unwrap();

        assert_eq!(0xFF, state.cpu.get(Register::B).unwrap());
        assert_eq!(0xFF, state.cpu.get(Register::C).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());
        assert_eq!(0x00, state.cpu.get(Register::C).unwrap());
        assert!(state.cpu.has_flag(Flags::HALF_CARRY));
        assert!(state.cpu.has_flag(Flags::CARRY));
    }
}
