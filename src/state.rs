use crate::cpu::CPU;
use crate::mmu::MMU;

#[derive(Debug)]
pub struct State {
    pub cpu: CPU,
    pub mmu: MMU,
}

impl State {
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            mmu: MMU::new(),
        }
    }
}
