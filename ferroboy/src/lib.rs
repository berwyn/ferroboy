#![deny(nonstandard_style, rust_2018_idioms, future_incompatible)]
#![deny(clippy::all)]

pub use crate::{
    state::{State, StateBuilder},
    system::{Cartridge, CartridgeBuilder, Config, ConfigBuilder},
};

#[cfg(feature = "introspection")]
pub use crate::{operations::Operation, system::OPCODES};

#[cfg(feature = "disassembly")]
pub use crate::assembly::*;

use crate::{error::Error, system::WideRegister};

#[cfg(not(feature = "introspection"))]
use crate::system::OPCODES;

pub mod error;

mod assembly;
mod helpers;
mod operations;
mod state;
mod system;

pub type Result<T> = core::result::Result<T, crate::error::Error>;

/// Prepare the system and start the emulation.
pub fn start(state: &mut State) -> Result<()> {
    if state.cartridge.is_some() {
        return state.map_cartridge().map(|_| {
            state.jump(0x0100);
        });
    }

    Err(Error::StateNotReady)
}

/// Step the emulation.
///
/// In an ideal world, this should be done at the clock rate of the Gameboy, but technically
/// can be done at any rate.
pub fn tick(state: &mut State) -> Result<&'static dyn crate::operations::Operation> {
    let address = state.cpu.get16(WideRegister::Pc);
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

        operation.act(state).map(|_| *operation)
    } else {
        Err(Error::InvalidOperation(opcode))
    }
}
