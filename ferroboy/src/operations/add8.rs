use crate::cpu::{Flags, Register};
use crate::operations::Operation;
use crate::state::State;

pub struct Add8Operation(pub Register, pub u8);

impl Operation for Add8Operation {
    fn act(&self, state: &mut State) -> Result<(), String> {
        state.cpu.clear_flag(Flags::SUBTRACTION);
        state.cpu.mutate(self.0, |old| old + self.1)?;

        // TODO: H + C

        Ok(())
    }
}
