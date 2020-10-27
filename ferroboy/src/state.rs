use std::rc::Rc;

use crate::system::{Cartridge, Config, WideRegister, CPU, MMU};

/// The current state of the emulation.
///
/// This struct serves as the overall wrapper for the emulation system
/// and holds all the references to the various sub-systems.
#[derive(Debug)]
pub struct State {
    pub config: Config,
    pub cpu: CPU,
    pub mmu: MMU,
    pub cartridge: Rc<Option<Cartridge>>,
}

impl State {
    pub fn is_halted(&self) -> bool {
        self.cpu.is_halted()
    }

    pub fn load_cartridge(&mut self, cartridge: Cartridge) {
        let rc = Rc::new(Some(cartridge));
        self.mmu = MMU::new(rc.clone());
        self.cartridge = rc;
    }

    pub(crate) fn read_byte(&mut self) -> crate::Result<u8> {
        let pc = self.cpu.get16(WideRegister::PC);
        let word = self.mmu[pc];

        self.increment_program_counter()?;

        Ok(word)
    }

    pub(crate) fn read_word(&mut self) -> crate::Result<(u8, u8)> {
        let high = self.read_byte()?;
        let low = self.read_byte()?;

        Ok((high, low))
    }

    pub(crate) fn increment_program_counter(&mut self) -> crate::Result<u16> {
        let pointer = self.cpu.get16(WideRegister::PC);
        match pointer.checked_add(1) {
            Some(new_pointer) => {
                self.cpu.set16(WideRegister::PC, new_pointer);
                Ok(new_pointer)
            }
            None => Err(crate::error::Error::AddressOutOfRange((pointer as u32) + 1)),
        }
    }

    pub(crate) fn jump(&mut self, destination: u16) -> crate::Result<()> {
        // FIXME(berwyn): Validate jump target
        self.cpu.set16(WideRegister::PC, destination);

        Ok(())
    }

    pub(crate) fn map_cartridge(&mut self) -> crate::Result<()> {
        if let Some(cart) = self.cartridge.as_ref() {
            cart.load_banks(&mut self.mmu);
        }

        Ok(())
    }
}

impl Default for State {
    fn default() -> Self {
        let cartridge = Rc::new(None);

        Self {
            config: Config::default(),
            cpu: CPU::default(),
            mmu: MMU::new(cartridge.clone()),
            cartridge,
        }
    }
}

#[derive(Default)]
pub struct StateBuilder {
    config: Config,
    cartridge: Option<Cartridge>,
}

impl StateBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }

    pub fn with_cartridge(mut self, cartridge: Cartridge) -> Self {
        self.cartridge.replace(cartridge);
        self
    }

    pub fn build(self) -> State {
        let rc = Rc::new(self.cartridge);

        State {
            config: self.config,
            cpu: Default::default(),
            mmu: MMU::new(rc.clone()),
            cartridge: rc,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_a_byte() {
        let mut state = State::default();

        state.cpu.set16(WideRegister::PC, 0x00);
        state.mmu.mutate(|mmu| mmu[0x00] = 0xFE);

        let byte = state.read_byte().unwrap();

        assert_eq!(0xFE, byte);
        assert_eq!(0x01, state.cpu.get16(WideRegister::PC));
    }

    #[test]
    fn it_reads_a_word() {
        let mut state = State::default();

        state.cpu.set16(WideRegister::PC, 0x00);
        state.mmu.mutate(|mmu| mmu[0x00] = 0xBE);
        state.mmu.mutate(|mmu| mmu[0x01] = 0xEF);

        let word = state.read_word().unwrap();

        assert_eq!((0xBE, 0xEF), word);
        assert_eq!(0x02, state.cpu.get16(WideRegister::PC));
    }
}
