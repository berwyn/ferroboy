use arrayvec::ArrayVec;

#[derive(Debug, Default)]
pub struct MMU {
    pub bios: ArrayVec<[u8; 256]>,
    pub rom: ArrayVec<[u8; 16_384]>,
    pub vram: ArrayVec<[u8; 8192]>,
    pub wram: ArrayVec<[u8; 4096]>,
}

impl MMU {
    pub fn new() -> Self {
        Self {
            bios: ArrayVec::new(),
            rom: ArrayVec::new(),
            vram: ArrayVec::new(),
            wram: ArrayVec::new(),
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
