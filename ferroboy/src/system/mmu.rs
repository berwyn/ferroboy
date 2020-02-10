use std::ops::{Index, IndexMut};

pub struct MMU {
    /// The first ROM bank, fixed, in cart
    bank0: [u8; 16_384],
    /// The second ROM bank, switchable, in cart
    bank1: [u8; 16_384],
    /// VRAM, fixed for DMG, switchable in CGB
    vram: [u8; 8_192],
    /// Switchable external RAM, in cart
    eram: [u8; 8_192],
    /// Work RAM, fixed
    wram0: [u8; 4_096],
    /// Work RAM, switchable in CGB mode
    wram1: [u8; 4_096],
    /// Sprite attribute table
    oam: [u8; 160],
    io: [u8; 128],
    hram: [u8; 127],
    fake: u8, // Use this to deal with unusable address spaces
}

impl MMU {
    pub fn new() -> Self {
        Self {
            bank0: [0; 16_384],
            bank1: [0; 16_384],
            vram: [0; 8_192],
            eram: [0; 8_192],
            wram0: [0; 4_096],
            wram1: [0; 4_096],
            oam: [0; 160],
            io: [0; 128],
            hram: [0; 127],
            fake: 0,
        }
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

    fn index(&self, index: u16) -> &Self::Output {
        let actual_index = index as usize;
        match index {
            0x0000..=0x3FFF => &self.bank0[actual_index],
            0x4000..=0x7FFF => &self.bank1[actual_index - 0x3FFF],
            0x8000..=0x9FFF => &self.vram[actual_index - 0x8000],
            0xA000..=0xBFFF => &self.eram[actual_index - 0xA000],
            0xC000..=0xCFFF => &self.wram0[actual_index - 0xC000],
            0xD000..=0xDFFF => &self.wram1[actual_index - 0xD000],
            0xE000..=0xFDFF => &self.wram0[actual_index - 0xE000],
            0xFE00..=0xFE9F => &self.oam[actual_index - 0xFE00],
            0xFEA0..=0xFEFF => &0, // not usable on actual hardware
            0xFF00..=0xFF7F => &self.io[actual_index - 0xFF00],
            0xFF80..=0xFFFE => &self.hram[actual_index - 0xFF80],
            0xFFFF => &0, // TODO(berwyn): Wire up to interrupt enabled/disabled
        }
    }
}

impl IndexMut<u16> for MMU {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        let actual_index = index as usize;
        match index {
            0x0000..=0x3FFF => &mut self.bank0[actual_index],
            0x4000..=0x7FFF => &mut self.bank1[actual_index - 0x3FFF],
            0x8000..=0x9FFF => &mut self.vram[actual_index - 0x8000],
            0xA000..=0xBFFF => &mut self.eram[actual_index - 0xA000],
            0xC000..=0xCFFF => &mut self.wram0[actual_index - 0xC000],
            0xD000..=0xDFFF => &mut self.wram1[actual_index - 0xD000],
            0xE000..=0xFDFF => &mut self.wram0[actual_index - 0xE000],
            0xFE00..=0xFE9F => &mut self.oam[actual_index - 0xFE00],
            0xFEA0..=0xFEFF => &mut self.fake, // Not usable on actual hardware
            0xFF00..=0xFF7F => &mut self.io[actual_index - 0xFF00],
            0xFF80..=0xFFFE => &mut self.hram[actual_index - 0xFF80],
            0xFFFF => &mut self.fake, // TODO(berwyn): Wire up to interrupt enabled/disabled
        }
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
