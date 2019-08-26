use crate::cpu::{Register, CPU};
use crate::operations::Operation;
use crate::state::State;

pub struct Load16ImmediateOperation(pub Register, pub u16);

impl Operation for Load16ImmediateOperation {
    fn act(&self, state: &mut State) -> Result<(), String> {
        let (high, low) = CPU::reg16_to_reg8(self.0)?;

        let high_value = (self.1 >> 8) as u8;
        let low_value = (self.1 & 0xFF) as u8;

        state.cpu.set(high, |_| high_value)?;
        state.cpu.set(low, |_| low_value)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_loads_an_immediate_into_the_registers() {
        let mut state = State::new();
        let op = Load16ImmediateOperation(Register::BC, 0xBEEF);

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());
        assert_eq!(0x00, state.cpu.get(Register::C).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0xBE, state.cpu.get(Register::B).unwrap());
        assert_eq!(0xEF, state.cpu.get(Register::C).unwrap());
    }
}
