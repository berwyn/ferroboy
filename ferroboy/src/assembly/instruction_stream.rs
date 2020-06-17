use crate::system::Cartridge;

pub struct AssemblyInstructionStream<'a> {
    cartridge: &'a Cartridge,
    pointer: usize,
}

impl<'a> AssemblyInstructionStream<'a> {
    pub(crate) fn new(cartridge: &'a Cartridge) -> Self {
        Self {
            cartridge,
            pointer: 0x100, // DMG-01 assembly starts at 0x100
        }
    }
}

impl Iterator for AssemblyInstructionStream<'_> {
    type Item = super::AssemblyInstruction;

    fn next(&mut self) -> Option<Self::Item> {
        let opcode = self.cartridge.data[self.pointer];
        if let Some(operation) = crate::OPCODES.get(&opcode) {
            match operation.disassemble(self.cartridge, self.pointer) {
                Ok(instruction) => todo!(),
                Err(_) => None,
            }
        } else {
            None
        }
    }
}
