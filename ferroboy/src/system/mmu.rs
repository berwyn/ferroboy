use std::ops::{Index, IndexMut};

pub struct MMU {
    memory: [u8; 0x10000],
}

impl MMU {
    pub fn new() -> Self {
        Self {
            memory: [0; 0x10000],
        }
    }

    pub fn bank0(&self) -> &[u8] {
        &self.memory[0x0000..=0x3FFF]
    }

    pub fn bank0_mut(&mut self) -> &mut [u8] {
        &mut self.memory[0x0000..=0x3FFF]
    }

    pub fn bank1(&self) -> &[u8] {
        &self.memory[0x4000..=0x7FFF]
    }

    pub fn bank1_mut(&mut self) -> &mut [u8] {
        &mut self.memory[0x4000..=0x7FFF]
    }

    pub fn game_link(&self) -> &[u8] {
        &self.memory[0xFF01..=0xFF02]
    }

    pub fn game_link_mut(&mut self) -> &mut [u8] {
        &mut self.memory[0xFF01..=0xFF02]
    }
}

impl Default for MMU {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for MMU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MMU {{ }}")
    }
}

impl Index<u16> for MMU {
    type Output = u8;
    fn index(&self, address: u16) -> &Self::Output {
        &self.memory[address as usize]
    }
}

impl IndexMut<u16> for MMU {
    fn index_mut(&mut self, address: u16) -> &mut Self::Output {
        &mut self.memory[address as usize]
    }
}

#[cfg(test)]
impl MMU {
    pub fn mutate<F>(&mut self, mutator: F)
    where
        F: FnOnce(&mut MMU),
    {
        mutator(self);
    }
}