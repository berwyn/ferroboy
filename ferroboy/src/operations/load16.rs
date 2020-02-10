use crate::helpers::word_to_u16;
use crate::operations::Operation;
use crate::state::State;
use crate::system::Register;

pub struct Load16ImmediateOperation(pub Register);

impl Operation for Load16ImmediateOperation {
    fn act(&self, state: &mut State) -> Result<(), String> {
        let word = word_to_u16(state.read_word()?);
        state.cpu.set16(self.0, word)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_loads_an_immediate_into_the_registers() {
        let mut state = State::new();
        state.mmu.mutate(|m| {
            m[0x00] = 0xBE;
            m[0x01] = 0xEF;
        });

        let op = Load16ImmediateOperation(Register::BC);

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());
        assert_eq!(0x00, state.cpu.get(Register::C).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0xBE, state.cpu.get(Register::B).unwrap());
        assert_eq!(0xEF, state.cpu.get(Register::C).unwrap());
    }
}
