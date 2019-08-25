use crate::cpu::Register;
use crate::operations::Operation;
use crate::state::State;

/// Increments a single 8bit register.
/// Does not affect flags.
pub struct Inc8Operation(pub Register);

impl Operation for Inc8Operation {
    fn act(&self, state: &mut State) -> Result<(), String> {
        let mut temp = u16::from(state.cpu.get(self.0)?);
        temp += 1;

        state.cpu.set(self.0, |_| temp as u8)?;

        Ok(())
    }
}
