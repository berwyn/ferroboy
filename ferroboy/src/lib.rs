#![deny(nonstandard_style, rust_2018_idioms, future_incompatible)]
#![deny(clippy::all)]

pub use crate::state::State;
pub use crate::state::StateBuilder;
pub use crate::system::ConfigBuilder;
pub use crate::system::Register;
pub use crate::system::OPCODES;

mod assembly;
mod helpers;
mod operations;
mod state;
mod system;

pub type Result<T> = core::result::Result<T, String>;

pub fn start(state: &mut State) -> Result<()> {
    if let Some(_cart) = &state.cartridge {
        return state
            .map_cartridge()
            .and(state.jump(0x0100))
            .and_then(|_| Ok(()));
    }

    Err("Cartridge not loaded!".into())
}

pub fn tick(state: &mut State) -> Result<&'static dyn crate::operations::Operation> {
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

        operation.act(state).and_then(|_| Ok(*operation))
    } else {
        Err(format!("Invalid opcode! PC: {}", opcode))
    }
}
