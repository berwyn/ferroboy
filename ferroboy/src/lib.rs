#![deny(nonstandard_style, rust_2018_idioms, future_incompatible)]
#![deny(clippy::all)]

use std::collections::BTreeMap;

use once_cell::sync::Lazy;

use crate::cpu::Register;
use crate::operations::{
    compose_operations, Add8Operation, Inc16Operation, Inc8Operation, IncrementClockOperation,
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

    OPCODES.get(&opcode).expect("Invalid opcode").act(state)
}

fn leak<T>(value: T) -> &'static T {
    Box::leak(Box::new(value))
}

static OPCODES: Lazy<BTreeMap<u8, &'static dyn Operation>> = Lazy::new(|| {
    let mut map = BTreeMap::<u8, &'static dyn Operation>::new();

    map.insert(0x00, Box::leak(Box::new(IncrementClockOperation(4))))
        .unwrap();

    map.insert(
        0x01,
        leak(compose_operations(
            Load16ImmediateOperation(Register::BC),
            IncrementClockOperation(12),
        )),
    );

    map.insert(
        0x02,
        leak(compose_operations(
            Load8RegisterToMemoryOperation(Register::BC, Register::A),
            IncrementClockOperation(8),
        )),
    );

    map.insert(
        0x03,
        leak(compose_operations(
            Inc16Operation(Register::BC),
            IncrementClockOperation(8),
        )),
    );

    map.insert(
        0x04,
        leak(compose_operations(
            Inc8Operation(Register::B),
            IncrementClockOperation(4),
        )),
    );

    map.insert(
        0x06,
        leak(compose_operations(
            Load8ImmediateOperation(Register::B),
            IncrementClockOperation(8),
        )),
    );

    map.insert(
        0x0C,
        leak(compose_operations(
            Inc8Operation(Register::C),
            IncrementClockOperation(4),
        )),
    );

    map.insert(
        0x0E,
        leak(compose_operations(
            Load8ImmediateOperation(Register::C),
            IncrementClockOperation(8),
        )),
    );

    map
});

/*
fn parse_opcode(opcode: u8, state: &mut State) -> Result<&dyn Operation, String> {
    match opcode {
        0x11 => compose_operations(
            Load16ImmediateOperation(Register::DE, word_to_u16(state.read_word()?)),
            IncrementClockOperation(12),
        ),
        0x12 => compose_operations(
            Load8RegisterToMemoryOperation(Register::DE, Register::A),
            IncrementClockOperation(8),
        ),
        0x13 => compose_operations(Inc16Operation(Register::DE), IncrementClockOperation(8)),
        0x14 => compose_operations(Inc8Operation(Register::D), IncrementClockOperation(4)),
        0x16 => compose_operations(
            Load8ImmediateOperation(Register::D, state.read_byte()?),
            IncrementClockOperation(8),
        ),
        0x1C => compose_operations(Inc8Operation(Register::E), IncrementClockOperation(4)),
        0x1E => compose_operations(
            Load8ImmediateOperation(Register::E, state.read_byte()?),
            IncrementClockOperation(8),
        ),
        0x21 => compose_operations(
            Load16ImmediateOperation(Register::HL, word_to_u16(state.read_word()?)),
            IncrementClockOperation(12),
        ),
        // TODO(berwyn): ld (hl+), A <-- what is HL+?
        0x23 => compose_operations(Inc16Operation(Register::HL), IncrementClockOperation(8)),
        0x24 => compose_operations(Inc8Operation(Register::H), IncrementClockOperation(4)),
        0x26 => compose_operations(
            Load8ImmediateOperation(Register::H, state.read_byte()?),
            IncrementClockOperation(8),
        ),
        0x2C => compose_operations(Inc8Operation(Register::L), IncrementClockOperation(4)),
        0x2E => compose_operations(
            Load8ImmediateOperation(Register::L, state.read_byte()?),
            IncrementClockOperation(8),
        ),
        0x31 => compose_operations(
            Load16ImmediateOperation(Register::SP, word_to_u16(state.read_word()?)),
            IncrementClockOperation(12),
        ),
        // TODO(berwyn): ld (hl-), A <-- what is HL-?
        0x33 => compose_operations(Inc16Operation(Register::SP), IncrementClockOperation(8)),
        0x3E => compose_operations(
            Load8ImmediateOperation(Register::A, state.read_byte()?),
            IncrementClockOperation(8),
        ),
        0x40 => compose_operations(
            Load8RegisterCopyOperation(Register::B, Register::B),
            IncrementClockOperation(4),
        ),
        0x41 => compose_operations(
            Load8RegisterCopyOperation(Register::B, Register::C),
            IncrementClockOperation(4),
        ),
        0x42 => compose_operations(
            Load8RegisterCopyOperation(Register::B, Register::D),
            IncrementClockOperation(4),
        ),
        0x43 => compose_operations(
            Load8RegisterCopyOperation(Register::B, Register::E),
            IncrementClockOperation(4),
        ),
        0x44 => compose_operations(
            Load8RegisterCopyOperation(Register::B, Register::H),
            IncrementClockOperation(4),
        ),
        0x45 => compose_operations(
            Load8RegisterCopyOperation(Register::B, Register::L),
            IncrementClockOperation(4),
        ),
        0x46 => compose_operations(
            Load8FromMemoryOperation(Register::B, Register::HL),
            IncrementClockOperation(8),
        ),
        0x47 => compose_operations(
            Load8RegisterCopyOperation(Register::B, Register::A),
            IncrementClockOperation(4),
        ),
        0x48 => compose_operations(
            Load8RegisterCopyOperation(Register::C, Register::B),
            IncrementClockOperation(4),
        ),
        0x49 => compose_operations(
            Load8RegisterCopyOperation(Register::C, Register::C),
            IncrementClockOperation(4),
        ),
        0x4A => compose_operations(
            Load8RegisterCopyOperation(Register::C, Register::D),
            IncrementClockOperation(4),
        ),
        0x4B => compose_operations(
            Load8RegisterCopyOperation(Register::C, Register::E),
            IncrementClockOperation(4),
        ),
        0x4C => compose_operations(
            Load8RegisterCopyOperation(Register::C, Register::H),
            IncrementClockOperation(4),
        ),
        0x4D => compose_operations(
            Load8RegisterCopyOperation(Register::C, Register::L),
            IncrementClockOperation(4),
        ),
        0x4F => compose_operations(
            Load8RegisterCopyOperation(Register::C, Register::A),
            IncrementClockOperation(4),
        ),
        0x70 => compose_operations(
            Load8RegisterToMemoryOperation(Register::HL, Register::B),
            IncrementClockOperation(8),
        ),
        0x71 => compose_operations(
            Load8RegisterToMemoryOperation(Register::HL, Register::C),
            IncrementClockOperation(8),
        ),
        0x72 => compose_operations(
            Load8RegisterToMemoryOperation(Register::HL, Register::D),
            IncrementClockOperation(8),
        ),
        0x73 => compose_operations(
            Load8RegisterToMemoryOperation(Register::HL, Register::E),
            IncrementClockOperation(8),
        ),
        0x74 => compose_operations(
            Load8RegisterToMemoryOperation(Register::HL, Register::H),
            IncrementClockOperation(8),
        ),
        0x75 => compose_operations(
            Load8RegisterToMemoryOperation(Register::HL, Register::L),
            IncrementClockOperation(8),
        ),
        0x77 => compose_operations(
            Load8RegisterToMemoryOperation(Register::HL, Register::A),
            IncrementClockOperation(8),
        ),
        0x80 => compose_operations(
            Add8Operation(Register::A, state.cpu.get(Register::B)?),
            IncrementClockOperation(4),
        ),
        0xE2 => compose_operations(
            Load8RegisterToMemoryOperation(Register::C, Register::A),
            IncrementClockOperation(8),
        ),
        _ => Err("Bad opcode".into()),
    }
}
*/
