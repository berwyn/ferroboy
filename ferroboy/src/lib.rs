#![deny(nonstandard_style, rust_2018_idioms, future_incompatible)]
#![deny(clippy::all)]

use crate::cpu::Register;
use crate::helpers::word_to_u16;
use crate::operations::{
    Add8Operation, Inc16Operation, Inc8Operation, IncrementClockOperation,
    Load16ImmediateOperation, Load8FromMemoryOperation, Load8ImmediateOperation,
    Load8RegisterCopyOperation, Load8RegisterToMemoryOperation, Operation,
};
pub use crate::state::State;

mod cpu;
mod helpers;
mod mmu;
mod operations;
mod state;

pub fn tick(state: &mut State) -> Result<(), String> {
    let opcode = 0x00;
    let ops = parse_opcode(opcode, state)?;

    for op in ops {
        op.act(state)?;
    }

    Ok(())
}

fn parse_opcode(opcode: u8, state: &mut State) -> Result<Vec<Box<dyn Operation>>, String> {
    let op: Vec<Box<dyn Operation>> = match opcode {
        0x00 => vec![Box::new(IncrementClockOperation(4))],
        0x01 => vec![
            Box::new(Load16ImmediateOperation(
                Register::BC,
                word_to_u16(state.read_word()?),
            )),
            Box::new(IncrementClockOperation(12)),
        ],
        0x02 => vec![
            Box::new(Load8RegisterToMemoryOperation(Register::BC, Register::A)),
            Box::new(IncrementClockOperation(8)),
        ],
        0x03 => vec![
            Box::new(Inc16Operation(Register::BC)),
            Box::new(IncrementClockOperation(8)),
        ],
        0x04 => vec![
            Box::new(Inc8Operation(Register::B)),
            Box::new(IncrementClockOperation(4)),
        ],
        0x06 => vec![
            Box::new(Load8ImmediateOperation(Register::B, state.read_byte()?)),
            Box::new(IncrementClockOperation(8)),
        ],
        0x0C => vec![
            Box::new(Inc8Operation(Register::C)),
            Box::new(IncrementClockOperation(4)),
        ],
        0x0E => vec![
            Box::new(Load8ImmediateOperation(Register::C, state.read_byte()?)),
            Box::new(IncrementClockOperation(8)),
        ],
        0x11 => vec![
            Box::new(Load16ImmediateOperation(
                Register::DE,
                word_to_u16(state.read_word()?),
            )),
            Box::new(IncrementClockOperation(12)),
        ],
        0x12 => vec![
            Box::new(Load8RegisterToMemoryOperation(Register::DE, Register::A)),
            Box::new(IncrementClockOperation(8)),
        ],
        0x13 => vec![
            Box::new(Inc16Operation(Register::DE)),
            Box::new(IncrementClockOperation(8)),
        ],
        0x14 => vec![
            Box::new(Inc8Operation(Register::D)),
            Box::new(IncrementClockOperation(4)),
        ],
        0x16 => vec![
            Box::new(Load8ImmediateOperation(Register::D, state.read_byte()?)),
            Box::new(IncrementClockOperation(8)),
        ],
        0x1C => vec![
            Box::new(Inc8Operation(Register::E)),
            Box::new(IncrementClockOperation(4)),
        ],
        0x1E => vec![
            Box::new(Load8ImmediateOperation(Register::E, state.read_byte()?)),
            Box::new(IncrementClockOperation(8)),
        ],
        0x21 => vec![
            Box::new(Load16ImmediateOperation(
                Register::HL,
                word_to_u16(state.read_word()?),
            )),
            Box::new(IncrementClockOperation(12)),
        ],
        // TODO(berwyn): ld (hl+), A <-- what is HL+?
        0x23 => vec![
            Box::new(Inc16Operation(Register::HL)),
            Box::new(IncrementClockOperation(8)),
        ],
        0x24 => vec![
            Box::new(Inc8Operation(Register::H)),
            Box::new(IncrementClockOperation(4)),
        ],
        0x26 => vec![
            Box::new(Load8ImmediateOperation(Register::H, state.read_byte()?)),
            Box::new(IncrementClockOperation(8)),
        ],
        0x2C => vec![
            Box::new(Inc8Operation(Register::L)),
            Box::new(IncrementClockOperation(4)),
        ],
        0x2E => vec![
            Box::new(Load8ImmediateOperation(Register::L, state.read_byte()?)),
            Box::new(IncrementClockOperation(8)),
        ],
        0x31 => vec![
            Box::new(Load16ImmediateOperation(
                Register::SP,
                word_to_u16(state.read_word()?),
            )),
            Box::new(IncrementClockOperation(12)),
        ],
        // TODO(berwyn): ld (hl-), A <-- what is HL-?
        0x33 => vec![
            Box::new(Inc16Operation(Register::SP)),
            Box::new(IncrementClockOperation(8)),
        ],
        0x3E => vec![
            Box::new(Load8ImmediateOperation(Register::A, state.read_byte()?)),
            Box::new(IncrementClockOperation(8)),
        ],
        0x40 => vec![
            Box::new(Load8RegisterCopyOperation(Register::B, Register::B)),
            Box::new(IncrementClockOperation(4)),
        ],
        0x41 => vec![
            Box::new(Load8RegisterCopyOperation(Register::B, Register::C)),
            Box::new(IncrementClockOperation(4)),
        ],
        0x42 => vec![
            Box::new(Load8RegisterCopyOperation(Register::B, Register::D)),
            Box::new(IncrementClockOperation(4)),
        ],
        0x43 => vec![
            Box::new(Load8RegisterCopyOperation(Register::B, Register::E)),
            Box::new(IncrementClockOperation(4)),
        ],
        0x44 => vec![
            Box::new(Load8RegisterCopyOperation(Register::B, Register::H)),
            Box::new(IncrementClockOperation(4)),
        ],
        0x45 => vec![
            Box::new(Load8RegisterCopyOperation(Register::B, Register::L)),
            Box::new(IncrementClockOperation(4)),
        ],
        0x46 => vec![
            Box::new(Load8FromMemoryOperation(Register::B, Register::HL)),
            Box::new(IncrementClockOperation(8)),
        ],
        0x47 => vec![
            Box::new(Load8RegisterCopyOperation(Register::B, Register::A)),
            Box::new(IncrementClockOperation(4)),
        ],
        0x48 => vec![
            Box::new(Load8RegisterCopyOperation(Register::C, Register::B)),
            Box::new(IncrementClockOperation(4)),
        ],
        0x49 => vec![
            Box::new(Load8RegisterCopyOperation(Register::C, Register::C)),
            Box::new(IncrementClockOperation(4)),
        ],
        0x4A => vec![
            Box::new(Load8RegisterCopyOperation(Register::C, Register::D)),
            Box::new(IncrementClockOperation(4)),
        ],
        0x4B => vec![
            Box::new(Load8RegisterCopyOperation(Register::C, Register::E)),
            Box::new(IncrementClockOperation(4)),
        ],
        0x4C => vec![
            Box::new(Load8RegisterCopyOperation(Register::C, Register::H)),
            Box::new(IncrementClockOperation(4)),
        ],
        0x4D => vec![
            Box::new(Load8RegisterCopyOperation(Register::C, Register::L)),
            Box::new(IncrementClockOperation(4)),
        ],
        0x4F => vec![
            Box::new(Load8RegisterCopyOperation(Register::C, Register::A)),
            Box::new(IncrementClockOperation(4)),
        ],
        0x70 => vec![
            Box::new(Load8RegisterToMemoryOperation(Register::HL, Register::B)),
            Box::new(IncrementClockOperation(8)),
        ],
        0x71 => vec![
            Box::new(Load8RegisterToMemoryOperation(Register::HL, Register::C)),
            Box::new(IncrementClockOperation(8)),
        ],
        0x72 => vec![
            Box::new(Load8RegisterToMemoryOperation(Register::HL, Register::D)),
            Box::new(IncrementClockOperation(8)),
        ],
        0x73 => vec![
            Box::new(Load8RegisterToMemoryOperation(Register::HL, Register::E)),
            Box::new(IncrementClockOperation(8)),
        ],
        0x74 => vec![
            Box::new(Load8RegisterToMemoryOperation(Register::HL, Register::H)),
            Box::new(IncrementClockOperation(8)),
        ],
        0x75 => vec![
            Box::new(Load8RegisterToMemoryOperation(Register::HL, Register::L)),
            Box::new(IncrementClockOperation(8)),
        ],
        0x77 => vec![
            Box::new(Load8RegisterToMemoryOperation(Register::HL, Register::A)),
            Box::new(IncrementClockOperation(8)),
        ],
        0x80 => vec![
            Box::new(Add8Operation(Register::A, state.cpu.get(Register::B)?)),
            Box::new(IncrementClockOperation(4)),
        ],
        0xE2 => vec![
            Box::new(Load8RegisterToMemoryOperation(Register::C, Register::A)),
            Box::new(IncrementClockOperation(8)),
        ],
        _ => return Err("Bad opcode".into()),
    };

    Ok(op)
}
