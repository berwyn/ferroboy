use std::collections::BTreeMap;

use once_cell::sync::Lazy;

use crate::cpu::Register;
use crate::operations::*;

type OpCodeMap = BTreeMap<u8, &'static dyn Operation>;

pub(crate) static OPCODES: Lazy<OpCodeMap> = Lazy::new(|| {
    let mut map = BTreeMap::new();

    load_rank_0_ops(&mut map);
    load_rank_1_ops(&mut map);
    load_rank_2_ops(&mut map);
    load_rank_3_ops(&mut map);
    load_rank_4_ops(&mut map);
    load_rank_7_ops(&mut map);
    load_rank_8_ops(&mut map);
    load_rank_E_ops(&mut map);

    map
});

fn leak<T>(value: T) -> &'static T {
    Box::leak(Box::new(value))
}

fn load_rank_0_ops(map: &mut OpCodeMap) {
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
}

fn load_rank_1_ops(map: &mut OpCodeMap) {
    map.insert(
        0x11,
        leak(compose_operations(
            Load16ImmediateOperation(Register::DE),
            IncrementClockOperation(12),
        )),
    );

    map.insert(
        0x12,
        leak(compose_operations(
            Load8RegisterToMemoryOperation(Register::DE, Register::A),
            IncrementClockOperation(8),
        )),
    );

    map.insert(
        0x13,
        leak(compose_operations(
            Inc16Operation(Register::DE),
            IncrementClockOperation(8),
        )),
    );

    map.insert(
        0x14,
        leak(compose_operations(
            Inc8Operation(Register::D),
            IncrementClockOperation(4),
        )),
    );

    map.insert(
        0x16,
        leak(compose_operations(
            Load8ImmediateOperation(Register::D),
            IncrementClockOperation(8),
        )),
    );

    map.insert(
        0x1C,
        leak(compose_operations(
            Inc8Operation(Register::E),
            IncrementClockOperation(4),
        )),
    );

    map.insert(
        0x1E,
        leak(compose_operations(
            Load8ImmediateOperation(Register::E),
            IncrementClockOperation(8),
        )),
    );
}

fn load_rank_2_ops(map: &mut OpCodeMap) {
    map.insert(
        0x21,
        leak(compose_operations(
            Load16ImmediateOperation(Register::HL),
            IncrementClockOperation(12),
        )),
    );

    // TODO(berwyn): 0x22 ld (hl+), A <-- what is HL+?

    map.insert(
        0x23,
        leak(compose_operations(
            Inc16Operation(Register::HL),
            IncrementClockOperation(8),
        )),
    );

    map.insert(
        0x24,
        leak(compose_operations(
            Inc8Operation(Register::H),
            IncrementClockOperation(4),
        )),
    );

    map.insert(
        0x26,
        leak(compose_operations(
            Load8ImmediateOperation(Register::H),
            IncrementClockOperation(8),
        )),
    );

    map.insert(
        0x2C,
        leak(compose_operations(
            Inc8Operation(Register::L),
            IncrementClockOperation(4),
        )),
    );

    map.insert(
        0x2E,
        leak(compose_operations(
            Load8ImmediateOperation(Register::L),
            IncrementClockOperation(8),
        )),
    );
}

fn load_rank_3_ops(map: &mut OpCodeMap) {
    map.insert(
        0x31,
        leak(compose_operations(
            Load16ImmediateOperation(Register::SP),
            IncrementClockOperation(12),
        )),
    );

    map.insert(
        0x33,
        leak(compose_operations(
            Inc16Operation(Register::SP),
            IncrementClockOperation(8),
        )),
    );

    // TODO(berwyn): ld (hl-), A <-- what is HL-?

    map.insert(
        0x3E,
        leak(compose_operations(
            Load8ImmediateOperation(Register::A),
            IncrementClockOperation(8),
        )),
    );
}

fn load_rank_4_ops(map: &mut OpCodeMap) {
    map.insert(
        0x40,
        leak(compose_operations(
            Load8RegisterCopyOperation(Register::B, Register::B),
            IncrementClockOperation(4),
        )),
    );

    map.insert(
        0x41,
        leak(compose_operations(
            Load8RegisterCopyOperation(Register::B, Register::C),
            IncrementClockOperation(4),
        )),
    );

    map.insert(
        0x42,
        leak(compose_operations(
            Load8RegisterCopyOperation(Register::B, Register::D),
            IncrementClockOperation(4),
        )),
    );

    map.insert(
        0x43,
        leak(compose_operations(
            Load8RegisterCopyOperation(Register::B, Register::E),
            IncrementClockOperation(4),
        )),
    );

    map.insert(
        0x44,
        leak(compose_operations(
            Load8RegisterCopyOperation(Register::B, Register::H),
            IncrementClockOperation(4),
        )),
    );

    map.insert(
        0x45,
        leak(compose_operations(
            Load8RegisterCopyOperation(Register::B, Register::L),
            IncrementClockOperation(4),
        )),
    );

    map.insert(
        0x46,
        leak(compose_operations(
            Load8FromMemoryOperation(Register::B, Register::HL),
            IncrementClockOperation(8),
        )),
    );

    map.insert(
        0x47,
        leak(compose_operations(
            Load8RegisterCopyOperation(Register::B, Register::A),
            IncrementClockOperation(4),
        )),
    );

    map.insert(
        0x48,
        leak(compose_operations(
            Load8RegisterCopyOperation(Register::C, Register::B),
            IncrementClockOperation(4),
        )),
    );

    map.insert(
        0x49,
        leak(compose_operations(
            Load8RegisterCopyOperation(Register::C, Register::C),
            IncrementClockOperation(4),
        )),
    );

    map.insert(
        0x4A,
        leak(compose_operations(
            Load8RegisterCopyOperation(Register::C, Register::D),
            IncrementClockOperation(4),
        )),
    );

    map.insert(
        0x4B,
        leak(compose_operations(
            Load8RegisterCopyOperation(Register::C, Register::E),
            IncrementClockOperation(4),
        )),
    );

    map.insert(
        0x4C,
        leak(compose_operations(
            Load8RegisterCopyOperation(Register::C, Register::H),
            IncrementClockOperation(4),
        )),
    );

    map.insert(
        0x4D,
        leak(compose_operations(
            Load8RegisterCopyOperation(Register::C, Register::L),
            IncrementClockOperation(4),
        )),
    );

    map.insert(
        0x4F,
        leak(compose_operations(
            Load8RegisterCopyOperation(Register::C, Register::A),
            IncrementClockOperation(4),
        )),
    );
}

fn load_rank_7_ops(map: &mut OpCodeMap) {
    map.insert(
        0x70,
        leak(compose_operations(
            Load8RegisterToMemoryOperation(Register::HL, Register::B),
            IncrementClockOperation(8),
        )),
    );

    map.insert(
        0x71,
        leak(compose_operations(
            Load8RegisterToMemoryOperation(Register::HL, Register::C),
            IncrementClockOperation(8),
        )),
    );

    map.insert(
        0x72,
        leak(compose_operations(
            Load8RegisterToMemoryOperation(Register::HL, Register::D),
            IncrementClockOperation(8),
        )),
    );

    map.insert(
        0x73,
        leak(compose_operations(
            Load8RegisterToMemoryOperation(Register::HL, Register::E),
            IncrementClockOperation(8),
        )),
    );

    map.insert(
        0x74,
        leak(compose_operations(
            Load8RegisterToMemoryOperation(Register::HL, Register::H),
            IncrementClockOperation(8),
        )),
    );

    map.insert(
        0x75,
        leak(compose_operations(
            Load8RegisterToMemoryOperation(Register::HL, Register::L),
            IncrementClockOperation(8),
        )),
    );

    map.insert(
        0x77,
        leak(compose_operations(
            Load8RegisterToMemoryOperation(Register::HL, Register::A),
            IncrementClockOperation(8),
        )),
    );
}

fn load_rank_8_ops(map: &mut OpCodeMap) {
    map.insert(
        0x80,
        leak(compose_operations(
            Add8Operation(Register::A, Register::B),
            IncrementClockOperation(4),
        )),
    );
}

#[allow(non_snake_case)] // `E` is a hex number here, not a letter
fn load_rank_E_ops(map: &mut OpCodeMap) {
    map.insert(
        0xE2,
        leak(compose_operations(
            Load8RegisterToMemoryOperation(Register::C, Register::A),
            IncrementClockOperation(8),
        )),
    );
}
