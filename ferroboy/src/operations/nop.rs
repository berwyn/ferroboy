use crate::operations::Operation;
use crate::State;

#[derive(Debug)]
pub struct NopOperation();

impl Operation for NopOperation {
    fn act(&self, _: &mut State) -> Result<(), String> {
        Ok(())
    }
}
