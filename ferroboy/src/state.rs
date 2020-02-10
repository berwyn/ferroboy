use crate::cpu::{Register, CPU};
use crate::mmu::MMU;
use crate::system::Cartridge;

#[derive(Debug, Default)]
pub struct State {
    pub cpu: CPU,
    pub mmu: MMU,
    pub cartridge: Option<Cartridge>,
}

impl State {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn read_byte(&mut self) -> Result<u8, String> {
        let pc = self.cpu.get16(Register::PC)?;
        let word = self.mmu[pc];

        self.increment_program_counter()?;

        Ok(word)
    }

    pub fn read_word(&mut self) -> Result<(u8, u8), String> {
        let mut pc = self.cpu.get16(Register::PC)?;
        let high = self.mmu[pc];

        pc = self.increment_program_counter()?;

        let low = self.mmu[pc];

        self.increment_program_counter()?;

        Ok((high, low))
    }

    pub fn increment_program_counter(&mut self) -> Result<u16, String> {
        self.cpu.mutate16(Register::PC, |old| old + 1)
    }

    pub fn load_cartridge_from_file(&mut self, path: &str) -> Result<(), String> {
        let file = std::fs::File::open(path).map_err(|_| "Couldn't open file".to_string())?;
        let cartridge = Cartridge::from_file(file)?;

        self.cartridge = Some(cartridge);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_a_byte() {
        let mut state = State::new();

        state.cpu.set16(Register::PC, 0x00).unwrap();
        state.mmu.mutate(|mmu| mmu[0x00] = 0xFE);

        let byte = state.read_byte().unwrap();

        assert_eq!(0xFE, byte);
        assert_eq!(0x01, state.cpu.get16(Register::PC).unwrap());
    }

    #[test]
    fn it_reads_a_word() {
        let mut state = State::new();

        state.cpu.set16(Register::PC, 0x00).unwrap();
        state.mmu.mutate(|mmu| mmu[0x00] = 0xBE);
        state.mmu.mutate(|mmu| mmu[0x01] = 0xEF);

        let word = state.read_word().unwrap();

        assert_eq!((0xBE, 0xEF), word);
        assert_eq!(0x02, state.cpu.get16(Register::PC).unwrap());
    }
}
