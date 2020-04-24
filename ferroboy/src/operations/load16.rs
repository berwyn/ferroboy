use crate::helpers::word_to_u16;
use crate::operations::Operation;
use crate::state::State;
use crate::system::WideRegister;

/// Loads an immediate 16-bit value into a register.
///
/// # Opcode Reference
/// ## Assembly definition
/// ```a
/// LD BC,##
/// ```
///
/// ## Runtime
/// | Metric | Size |
/// |:-------|:-----|
/// | Length | 3    |
/// | Cycles | 12   |
///
/// ## Flags
/// | Flag          | Value        |
/// |:--------------|:-------------|
/// | Zero          | Not Affected |
/// | Subtraction   | Not Affected |
/// | Half-Carry    | Not Affected |
/// | Carry         | Not Affected |
///
/// # Examples
/// ```rs
/// Load16ImmediateOperation(Register::BC)
/// ```
///
/// # Errors
/// - The operation will fail if provided an 8-bit register
#[derive(Debug)]
pub struct Load16ImmediateOperation(pub WideRegister);

impl Operation for Load16ImmediateOperation {
    fn act(&self, state: &mut State) -> crate::Result<()> {
        let word = word_to_u16(state.read_word()?);
        state.cpu.set16(self.0.into(), word)?;
        state.cpu.increment_clock(12);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::system::Register;

    #[test]
    fn it_loads_an_immediate_into_the_registers() {
        let mut state = State::default();
        state.mmu.mutate(|m| {
            m[0x00] = 0xBE;
            m[0x01] = 0xEF;
        });

        let op = Load16ImmediateOperation(WideRegister::BC);

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());
        assert_eq!(0x00, state.cpu.get(Register::C).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0xBE, state.cpu.get(Register::B).unwrap());
        assert_eq!(0xEF, state.cpu.get(Register::C).unwrap());
    }
}
