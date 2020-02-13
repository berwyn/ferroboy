#![deny(nonstandard_style, rust_2018_idioms, future_incompatible)]
#![deny(clippy::all)]

pub use crate::state::State;
pub use crate::system::Register;
pub use crate::system::OPCODES;

mod helpers;
mod operations;
mod state;
mod system;

pub fn start(state: &mut State) -> Result<(), String> {
    if let Some(_cart) = &state.cartridge {
        state.map_cartridge()?;
        state.jump(0x0100)?;
        return Ok(());
    }

    Err("Cartridge not loaded!".into())
}

pub fn tick(state: &mut State) -> Result<(), String> {
    let address = state.cpu.get16(Register::PC)?;
    let opcode = state.mmu[address];

    state.increment_program_counter()?;

    if cfg!(debug_assertions) {
        println!("tick:");
        println!("\t{:X}: {:X}", address, opcode);
    }

    if let Some(operation) = OPCODES.get(&opcode) {
        if cfg!(debug_assertions) {
            println!("\t{:?}", operation);
        }

        operation.act(state)
    } else {
        Err(format!("Invalid opcode! PC: {}", opcode))
    }
}
