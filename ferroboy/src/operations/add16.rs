use crate::operations::Operation;
use crate::state::State;
use crate::system::{Flags, Register};

#[derive(Debug)]
pub struct Add16Operation(Register, u16);

impl Operation for Add16Operation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        let (high, low) = self.0.to_8bit_pair()?;

        state.cpu.clear_flag(Flags::SUBTRACTION);

        let lower_arg = self.1 & 0xFF;
        let upper_arg = self.1 >> 8;

        let calculated_lower = u16::from(state.cpu.get(low)?) + lower_arg;
        state.cpu.set(low, calculated_lower as u8)?;

        if calculated_lower / 0xFF > 0 {
            state.cpu.set_flag(Flags::HALF_CARRY);
        }

        let calculated_upper =
            u16::from(state.cpu.get(high)?) + upper_arg + (calculated_lower / 0xFF);

        state.cpu.set(high, calculated_upper as u8)?;

        if calculated_upper / 0xFF > 0 {
            state.cpu.set_flag(Flags::CARRY);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_the_lower_byte() {
        let mut state = State::default();
        let op = Add16Operation(Register::BC, 0x0010);

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());
        assert_eq!(0x00, state.cpu.get(Register::C).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());
        assert_eq!(0x10, state.cpu.get(Register::C).unwrap());
        assert!(!state.cpu.has_flag(Flags::HALF_CARRY));
        assert!(!state.cpu.has_flag(Flags::CARRY));
    }

    #[test]
    fn it_adds_the_upper_byte() {
        let mut state = State::default();
        let op = Add16Operation(Register::BC, 0x0F10);

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());
        assert_eq!(0x00, state.cpu.get(Register::C).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0x0F, state.cpu.get(Register::B).unwrap());
        assert_eq!(0x10, state.cpu.get(Register::C).unwrap());
        assert!(!state.cpu.has_flag(Flags::HALF_CARRY));
        assert!(!state.cpu.has_flag(Flags::CARRY));
    }

    #[test]
    fn it_sets_half_carry() {
        let mut state = State::default();
        let op = Add16Operation(Register::BC, 0x00FF);

        state.cpu.set(Register::C, 1).unwrap();

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());
        assert_eq!(0x01, state.cpu.get(Register::C).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0x01, state.cpu.get(Register::B).unwrap());
        assert_eq!(0x00, state.cpu.get(Register::C).unwrap());
        assert!(state.cpu.has_flag(Flags::HALF_CARRY));
        assert!(!state.cpu.has_flag(Flags::CARRY));
    }

    #[test]
    fn it_sets_carry() {
        let mut state = State::default();
        let op = Add16Operation(Register::BC, 0x0001);

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
