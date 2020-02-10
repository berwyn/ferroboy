use crate::cpu::{Flags, Register};
use crate::operations::Operation;
use crate::state::State;

pub struct Add8Operation(pub Register, pub Register);

impl Operation for Add8Operation {
    fn act(&self, state: &mut State) -> Result<(), String> {
        let value = state.cpu.get(self.1)?;

        state.cpu.clear_flag(Flags::SUBTRACTION);
        state.cpu.mutate(self.0, |old| old + value)?;

        // TODO: H + C

        Ok(())
    }
}
