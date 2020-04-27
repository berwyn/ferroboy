use crate::assembly::AssemblyInstruction;
use crate::State;

pub trait Disassemble {
    fn disassemble(&self, state: &mut State) -> crate::Result<AssemblyInstruction>;
}
