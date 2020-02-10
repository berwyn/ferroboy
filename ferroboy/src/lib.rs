#![deny(nonstandard_style, rust_2018_idioms, future_incompatible)]
#![deny(clippy::all)]

use crate::opcodes::OPCODES;
pub use crate::state::State;

mod cpu;
mod helpers;
mod mmu;
mod opcodes;
mod operations;
mod state;
mod system;

pub fn tick(state: &mut State) -> Result<(), String> {
    let opcode = 0x00;

    OPCODES.get(&opcode).expect("Invalid opcode").act(state)
}
