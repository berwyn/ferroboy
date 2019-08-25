use crate::operations::Operation;
use crate::state::State;

pub struct IncrementClockOperation(pub u64);

impl Operation for IncrementClockOperation {
    fn act(&self, state: &mut State) -> Result<(), String> {
        state.cpu.set_clock(|clock| clock + self.0);

        Ok(())
    }
}
