use crate::system::{Cartridge, Config, WideRegister, CPU, MMU};

/// The current state of the emulation.
///
/// This struct serves as the overall wrapper for the emulation system
/// and holds all the references to the various sub-systems.
#[derive(Debug, Default)]
pub struct State {
    pub config: Config,
    pub cpu: CPU,
    pub mmu: MMU,
    pub cartridge: Option<Cartridge>,
}

impl State {
    pub(crate) fn read_byte(&mut self) -> crate::Result<u8> {
        let pc = self.cpu.get16(WideRegister::PC);
        let word = self.mmu[pc];

        self.increment_program_counter()?;

        Ok(word)
    }

    pub(crate) fn read_word(&mut self) -> crate::Result<(u8, u8)> {
        let mut pc = self.cpu.get16(WideRegister::PC);
        let high = self.mmu[pc];

        pc = self.increment_program_counter()?;

        let low = self.mmu[pc];

        self.increment_program_counter()?;

        Ok((high, low))
    }

    // ? Should this be in state or somewhere else?
    pub(crate) fn increment_program_counter(&mut self) -> crate::Result<u16> {
        let new_pointer = self.cpu.get16(WideRegister::PC) + 1;
        self.cpu.set16(WideRegister::PC, new_pointer);

        Ok(new_pointer)
    }

    pub(crate) fn jump(&mut self, destination: u16) -> crate::Result<()> {
        // FIXME(berwyn): Validate jump target
        self.cpu.set16(WideRegister::PC, destination);

        Ok(())
    }

    pub(crate) fn map_cartridge(&mut self) -> crate::Result<()> {
        if let Some(cart) = &self.cartridge {
            cart.load_banks(&mut self.mmu);
        }

        Ok(())
    }

    pub fn load_cartridge_from_file(&mut self, path: &str) -> crate::Result<()> {
        let file = std::fs::File::open(path).map_err(|_| "Couldn't open file".to_string())?;
        let cartridge = Cartridge::from_file(file, &self.config)?;

        self.cartridge = Some(cartridge);

        Ok(())
    }

    pub fn load_cartridge_from_buffer(&mut self, buffer: &[u8]) -> crate::Result<()> {
        let cartridge = Cartridge::from_buffer(buffer, &self.config)?;

        self.cartridge = Some(cartridge);

        Ok(())
    }

    pub fn is_halted(&self) -> bool {
        self.cpu.is_halted()
    }
}

#[derive(Default)]
pub struct StateBuilder {
    config: Config,
}

impl StateBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_config(&mut self, config: Config) -> &mut Self {
        self.config = config;
        self
    }

    pub fn build(&self) -> State {
        State {
            config: self.config.clone(),
            cpu: Default::default(),
            mmu: Default::default(),
            cartridge: None,
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
