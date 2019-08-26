#![deny(clippy::all)]

use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::cpu::Register;
use crate::helpers::word_to_u16;
use crate::operations::{
    Add8Operation, Inc16Operation, Inc8Operation, IncrementClockOperation,
    Load16ImmediateOperation, Operation,
};
use crate::state::State;

mod cpu;
mod helpers;
mod mmu;
mod operations;
mod state;

lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(State::new());
}

#[no_mangle]
extern "C" fn retro_init() {
    lazy_static::initialize(&STATE);
}

#[no_mangle]
extern "C" fn retro_run() {
    let mut lock = STATE.lock().unwrap();
    let state = &mut *lock;
    tick(state).unwrap();
}

fn tick(state: &mut State) -> Result<(), String> {
    let opcode = 0x00;
    let ops: Vec<Box<dyn Operation>> = match opcode {
        0x00 => vec![Box::new(IncrementClockOperation(4))],
        0x01 => vec![
            Box::new(Load16ImmediateOperation(
                Register::BC,
                word_to_u16(state.read_word()?),
            )),
            Box::new(IncrementClockOperation(12)),
        ],
        0x03 => vec![
            Box::new(Inc16Operation(Register::BC)),
            Box::new(IncrementClockOperation(8)),
        ],
        0x04 => vec![
            Box::new(Inc8Operation(Register::B)),
            Box::new(IncrementClockOperation(4)),
        ],
        0x0C => vec![
            Box::new(Inc8Operation(Register::C)),
            Box::new(IncrementClockOperation(4)),
        ],
        0x11 => vec![
            Box::new(Load16ImmediateOperation(
                Register::DE,
                word_to_u16(state.read_word()?),
            )),
            Box::new(IncrementClockOperation(12)),
        ],
        0x13 => vec![
            Box::new(Inc16Operation(Register::DE)),
            Box::new(IncrementClockOperation(8)),
        ],
        0x14 => vec![
            Box::new(Inc8Operation(Register::D)),
            Box::new(IncrementClockOperation(4)),
        ],
        0x1C => vec![
            Box::new(Inc8Operation(Register::E)),
            Box::new(IncrementClockOperation(4)),
        ],
        0x21 => vec![
            Box::new(Load16ImmediateOperation(
                Register::HL,
                word_to_u16(state.read_word()?),
            )),
            Box::new(IncrementClockOperation(12)),
        ],
        0x23 => vec![
            Box::new(Inc16Operation(Register::HL)),
            Box::new(IncrementClockOperation(8)),
        ],
        0x24 => vec![
            Box::new(Inc8Operation(Register::H)),
            Box::new(IncrementClockOperation(4)),
        ],
        0x2C => vec![
            Box::new(Inc8Operation(Register::L)),
            Box::new(IncrementClockOperation(4)),
        ],
        0x31 => vec![
            Box::new(Load16ImmediateOperation(
                Register::SP,
                word_to_u16(state.read_word()?),
            )),
            Box::new(IncrementClockOperation(12)),
        ],
        0x33 => vec![
            Box::new(Inc16Operation(Register::SP)),
            Box::new(IncrementClockOperation(8)),
        ],
        0x80 => vec![
            Box::new(Add8Operation(Register::A, state.cpu.get(Register::B)?)),
            Box::new(IncrementClockOperation(4)),
        ],
        _ => return Err("Bad opcode".into()),
    };

    for op in ops {
        op.act(state)?;
    }

    Ok(())
}
