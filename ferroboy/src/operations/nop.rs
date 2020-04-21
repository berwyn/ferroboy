use crate::operations::Operation;
use crate::State;

#[derive(Debug)]
pub struct NopOperation;

impl Operation for NopOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        state.cpu.increment_clock(4);
        Ok(())
    }
}
