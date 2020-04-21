use crate::operations::Operation;
use crate::state::State;
use crate::system::Register;

/// Increments a single 8bit register.
/// Does not affect flags.
#[derive(Debug)]
pub struct Inc8Operation(pub Register);

impl Operation for Inc8Operation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        let mut temp = u16::from(state.cpu.get(self.0)?);
        temp += 1;

        state.cpu.set(self.0, temp as u8)?;
        state.cpu.increment_clock(4);

        Ok(())
    }
}
