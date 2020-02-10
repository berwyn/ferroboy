use crate::operations::Operation;
use crate::state::State;
use crate::system::{Flags, Register};

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
