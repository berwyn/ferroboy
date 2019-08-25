use arrayvec::ArrayVec;

#[derive(Debug)]
pub struct MMU {
    bios: ArrayVec<[u8; 256]>,
    rom: ArrayVec<[u8; 16_384]>,
    vram: ArrayVec<[u8; 8192]>,
    wram: ArrayVec<[u8; 4096]>,
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
