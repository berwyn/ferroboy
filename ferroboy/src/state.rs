use crate::cpu::{Register, CPU};
use crate::mmu::MMU;

#[derive(Debug, Default)]
pub struct State {
    pub cpu: CPU,
    pub mmu: MMU,
}

impl State {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn read_byte(&mut self) -> Result<u8, String> {
        let pc = self.cpu.get16(Register::PC)?;
        let word = self.mmu[pc];

        self.cpu.set16(Register::PC, |old| old + 1)?;

        Ok(word)
    }

    pub fn read_word(&mut self) -> Result<(u8, u8), String> {
        let mut pc = self.cpu.get16(Register::PC)?;
        let high = self.mmu[pc];

        pc = self.cpu.set16(Register::PC, |old| old + 1)?;

        let low = self.mmu[pc];

        self.cpu.set16(Register::PC, |old| old + 1)?;

        Ok((high, low))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_a_byte() {
        let mut state = State::new();

        state.cpu.set16(Register::PC, |_| 0x00).unwrap();
        state.mmu.mutate(|mmu| mmu[0x00] = 0xFE);

        let byte = state.read_byte().unwrap();

        assert_eq!(0xFE, byte);
        assert_eq!(0x01, state.cpu.get16(Register::PC).unwrap());
    }

    #[test]
    fn it_reads_a_word() {
        let mut state = State::new();

        state.cpu.set16(Register::PC, |_| 0x00).unwrap();
        state.mmu.mutate(|mmu| mmu[0x00] = 0xBE);
        state.mmu.mutate(|mmu| mmu[0x01] = 0xEF);

        let word = state.read_word().unwrap();

        assert_eq!((0xBE, 0xEF), word);
        assert_eq!(0x02, state.cpu.get16(Register::PC).unwrap());
    }
}
