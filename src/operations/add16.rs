use crate::cpu::{Flags, Register, CPU};
use crate::operations::Operation;
use crate::state::State;

#[derive(Clone, Copy, Debug)]
pub struct Add16Operation(Register, u16);

impl Operation for Add16Operation {
    fn act(&self, state: &mut State) -> Result<(), String> {
        let (high, low) = CPU::reg16_to_reg8(self.0)?;

        state.cpu.clear_flag(Flags::SUBTRACTION);

        let lower_arg = self.1 & 0xFF;
        let upper_arg = self.1 >> 8;

        let calculated_lower = u16::from(state.cpu.get(low)?) + lower_arg;
        state.cpu.set(low, |_| calculated_lower as u8)?;

        if calculated_lower / 0xFF > 0 {
            state.cpu.set_flag(Flags::HALF_CARRY);
        }

        let calculated_upper =
            u16::from(state.cpu.get(high)?) + upper_arg + (calculated_lower / 0xFF);

        state.cpu.set(high, |_| calculated_upper as u8)?;

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
        let mut state = State::new();
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
        let mut state = State::new();
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
        let mut state = State::new();
        let op = Add16Operation(Register::BC, 0x00FF);

        state.cpu.set(Register::C, |_| 1).unwrap();

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
        let mut state = State::new();
        let op = Add16Operation(Register::BC, 0x0001);

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
