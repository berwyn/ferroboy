use crate::cpu::Register;
use crate::operations::Operation;
use crate::state::State;

pub struct Load16ImmediateOperation(pub Register, pub u16);

impl Operation for Load16ImmediateOperation {
    fn act(&self, state: &mut State) -> Result<(), String> {
        state.cpu.set16(self.0, self.1)?;

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
