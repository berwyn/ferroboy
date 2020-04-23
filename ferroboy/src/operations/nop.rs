use crate::operations::Operation;
use crate::State;

/// A non-operation.
///
/// # Opcode Reference
/// ## Assembly definition
/// ```a
/// NOP
/// ```
///
/// ## Runtime
/// | Metric | Size |
/// |:-------|:-----|
/// | Length | 1    |
/// | Cycles | 4    |
///
/// ## Flags
/// | Flag          | Value        |
/// |:--------------|:-------------|
/// | Zero          | Not Affected |
/// | Subtraction   | Not Affected |
/// | Half-Carry    | Not Affected |
/// | Carry         | Not Affected |
#[derive(Debug)]
pub struct NopOperation;

impl Operation for NopOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        state.cpu.increment_clock(4);
        Ok(())
    }
}
