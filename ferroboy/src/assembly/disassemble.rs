use crate::assembly::AssemblyInstruction;
use crate::system::Cartridge;

pub trait Disassemble {
    fn disassemble(
        &self,
        cartridge: &Cartridge,
        offset: usize,
    ) -> crate::Result<AssemblyInstruction>;
}
