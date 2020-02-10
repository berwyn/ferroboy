#![deny(nonstandard_style, rust_2018_idioms, future_incompatible)]
#![deny(clippy::all)]

pub use crate::state::State;
pub use crate::system::OPCODES;

mod helpers;
mod operations;
mod state;
mod system;

pub fn tick(state: &mut State) -> Result<(), String> {
    let opcode = 0x00;

    OPCODES.get(&opcode).expect("Invalid opcode").act(state)
}
