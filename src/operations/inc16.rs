use crate::cpu::{Flags, Register, CPU};
use crate::operations::Operation;
use crate::state::State;

/// Increments a singular 16bit register.
/// Does not affect flags.
pub struct Inc16Operation(pub Register);

impl Operation for Inc16Operation {
    fn act(&self, state: &mut State) -> Result<(), String> {
        match self.0 {
            Register::SP => state.cpu.set(self.0, |old| old + 1)?,
            _ => {
                let (high, low) = CPU::reg16_to_reg8(self.0)?;

                let mut lower = u16::from(state.cpu.get(low)?);
                lower += 1;
                state.cpu.set(low, |_| lower as u8)?;

                if lower / 0xFF > 0 {
                    state.cpu.set_flag(Flags::HALF_CARRY);
                    let mut upper = u16::from(state.cpu.get(high)?);
                    upper += 1;

                    if upper / 0xFF > 0 {
                        state.cpu.set_flag(Flags::CARRY);
                    }

                    state.cpu.set(high, |_| upper as u8)?;
                }
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_increments_the_lower_byte() {
        let mut state = State::new();
        let op = Inc16Operation(Register::BC);

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());
        assert_eq!(0x00, state.cpu.get(Register::C).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());
        assert_eq!(0x01, state.cpu.get(Register::C).unwrap());
    }

    #[test]
    fn it_increments_the_upper_byte() {
        let mut state = State::new();
        let op = Inc16Operation(Register::BC);

        state.cpu.set(Register::C, |_| 0xFF).unwrap();

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());
        assert_eq!(0xFF, state.cpu.get(Register::C).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0x01, state.cpu.get(Register::B).unwrap());
        assert_eq!(0x00, state.cpu.get(Register::C).unwrap());
        assert!(state.cpu.has_flag(Flags::HALF_CARRY));
    }

    #[test]
    fn it_wraps_over() {
        let mut state = State::new();
        let op = Inc16Operation(Register::BC);

        state.cpu.set(Register::B, |_| 0xFF).unwrap();
        state.cpu.set(Register::C, |_| 0xFF).unwrap();

        assert_eq!(0xFF, state.cpu.get(Register::B).unwrap());
        assert_eq!(0xFF, state.cpu.get(Register::C).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());
        assert_eq!(0x00, state.cpu.get(Register::C).unwrap());
        assert!(state.cpu.has_flag(Flags::HALF_CARRY));
        assert!(state.cpu.has_flag(Flags::CARRY));
    }
}
