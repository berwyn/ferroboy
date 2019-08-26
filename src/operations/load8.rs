use crate::cpu::Register;
use crate::operations::Operation;
use crate::state::State;

pub struct Load8ImmediateOperation(pub Register, pub u8);

impl Operation for Load8ImmediateOperation {
    fn act(&self, state: &mut State) -> Result<(), String> {
        state.cpu.set(self.0, |_| self.1).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_loads_an_immediate_into_the_register() {
        let mut state = State::new();
        let op = Load8ImmediateOperation(Register::B, 0xFE);

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0xFE, state.cpu.get(Register::B).unwrap());
    }
}
