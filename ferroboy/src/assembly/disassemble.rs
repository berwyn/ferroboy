use crate::{assembly::AssemblyInstruction, system::Cartridge};

pub trait Disassemble {
    fn disassemble(
        &self,
        cartridge: &Cartridge,
        offset: usize,
    ) -> crate::Result<AssemblyInstruction>;

    fn describe(&self) -> crate::Result<AssemblyInstruction>;
}
