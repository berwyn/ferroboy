use std::{rc::Rc, sync::Arc};

use crate::system::{Cartridge, Config, Cpu, Mmu, WideRegister};

/// The current state of the emulation.
///
/// This struct serves as the overall wrapper for the emulation system
/// and holds all the references to the various sub-systems.
#[derive(Debug)]
pub struct State {
    pub config: Config,
    pub cpu: Cpu,
    pub mmu: Mmu,
    pub cartridge: Arc<Option<Cartridge>>,
}

impl State {
    pub fn is_halted(&self) -> bool {
        self.cpu.is_halted()
    }

    pub fn load_cartridge(&mut self, cartridge: Cartridge) {
        let cart = Arc::new(Some(cartridge));
        self.mmu = Mmu::new(cart.clone());
        self.cartridge = cart;
    }

    pub(crate) fn read_byte(&mut self) -> crate::Result<u8> {
        let pc = self.cpu.get16(WideRegister::Pc);
        let word = self.mmu[pc];

        self.increment_program_counter()?;

        Ok(word)
    }

    pub(crate) fn read_word(&mut self) -> crate::Result<(u8, u8)> {
        let low = self.read_byte()?;
        let high = self.read_byte()?;

        Ok((high, low))
    }

    pub(crate) fn increment_program_counter(&mut self) -> crate::Result<u16> {
        let pointer = self.cpu.get16(WideRegister::Pc);
        match pointer.checked_add(1) {
            Some(new_pointer) => {
                self.cpu.set16(WideRegister::Pc, new_pointer);
                Ok(new_pointer)
            }
            None => Err(crate::error::Error::AddressOutOfRange((pointer as u32) + 1)),
        }
    }

    pub(crate) fn jump(&mut self, destination: u16) {
        // FIXME(berwyn): Validate jump target
        self.cpu.set16(WideRegister::Pc, destination);
    }

    pub(crate) fn map_cartridge(&mut self) -> crate::Result<()> {
        match self.cartridge.as_ref() {
            Some(cart) => {
                cart.load_banks(&mut self.mmu);
                Ok(())
            }
            None => Err(crate::Error::InvalidState),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        let cartridge = Arc::new(None);

        Self {
            config: Config::default(),
            cpu: Cpu::default(),
            mmu: Mmu::new(cartridge.clone()),
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
        let cart = Arc::new(self.cartridge);

        State {
            config: self.config,
            cpu: Default::default(),
            mmu: Mmu::new(cart.clone()),
            cartridge: cart,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_a_byte() {
        let mut state = State::default();

        state.cpu.set16(WideRegister::Pc, 0x00);
        state.mmu.mutate(|mmu| mmu[0x00] = 0xFE);

        let byte = state.read_byte().unwrap();

        assert_eq!(0xFE, byte);
        assert_eq!(0x01, state.cpu.get16(WideRegister::Pc));
    }

    #[test]
    fn it_reads_a_word() {
        let mut state = State::default();

        state.cpu.set16(WideRegister::Pc, 0x00);
        state.mmu.mutate(|mmu| mmu[0x00] = 0xBE);
        state.mmu.mutate(|mmu| mmu[0x01] = 0xEF);

        let word = state.read_word().unwrap();

        assert_eq!((0xBE, 0xEF), word);
        assert_eq!(0x02, state.cpu.get16(WideRegister::Pc));
    }
}
