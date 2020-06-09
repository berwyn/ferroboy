use std::collections::BTreeMap;

use once_cell::sync::Lazy;

use crate::operations::*;
use crate::system::{Register, WideRegister};

type OpCodeMap = BTreeMap<u8, &'static dyn Operation>;

/// A compile-time map of opcodes to their Operations.
pub static OPCODES: Lazy<OpCodeMap> = Lazy::new(|| {
    let mut map = BTreeMap::new();

    load_rank_0_ops(&mut map);
    load_rank_1_ops(&mut map);
    load_rank_2_ops(&mut map);
    load_rank_3_ops(&mut map);
    load_rank_4_ops(&mut map);
    load_rank_5_ops(&mut map);
    load_rank_6_ops(&mut map);
    load_rank_7_ops(&mut map);
    load_rank_8_ops(&mut map);
    load_rank_C_ops(&mut map);
    load_rank_D_ops(&mut map);
    load_rank_E_ops(&mut map);
    load_rank_F_ops(&mut map);

    map
});

/// Leak an object reference so that it lives on for the life of the program.
fn leak<T>(value: T) -> &'static T {
    Box::leak(Box::new(value))
}

fn load_rank_0_ops(map: &mut OpCodeMap) {
    map.insert(0x00, leak(NopOperation));

    map.insert(0x01, leak(Load16ImmediateOperation(WideRegister::BC)));

    map.insert(
        0x02,
        leak(Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::WideRegister(WideRegister::BC),
            Register::A,
        )),
    );

    map.insert(0x03, leak(Inc16Operation(WideRegister::BC)));

    map.insert(0x04, leak(Inc8Operation(Register::B)));

    map.insert(0x06, leak(Load8ImmediateOperation(Register::B)));

    map.insert(0x0C, leak(Inc8Operation(Register::C)));

    map.insert(0x0E, leak(Load8ImmediateOperation(Register::C)));
}

fn load_rank_1_ops(map: &mut OpCodeMap) {
    map.insert(0x00, leak(StopOperation));

    map.insert(0x11, leak(Load16ImmediateOperation(WideRegister::DE)));

    map.insert(
        0x12,
        leak(Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::WideRegister(WideRegister::DE),
            Register::A,
        )),
    );

    map.insert(0x13, leak(Inc16Operation(WideRegister::DE)));

    map.insert(0x14, leak(Inc8Operation(Register::D)));

    map.insert(0x16, leak(Load8ImmediateOperation(Register::D)));

    map.insert(0x18, leak(JumpRelativeOperation(JumpRelativeFlag::Nop)));

    map.insert(0x1C, leak(Inc8Operation(Register::E)));

    map.insert(0x1E, leak(Load8ImmediateOperation(Register::E)));
}

fn load_rank_2_ops(map: &mut OpCodeMap) {
    map.insert(0x20, leak(JumpRelativeOperation(JumpRelativeFlag::NotZero)));

    map.insert(0x21, leak(Load16ImmediateOperation(WideRegister::HL)));

    // TODO(berwyn): 0x22 ld (hl+), A <-- what is HL+?

    map.insert(0x23, leak(Inc16Operation(WideRegister::HL)));

    map.insert(0x24, leak(Inc8Operation(Register::H)));

    map.insert(0x26, leak(Load8ImmediateOperation(Register::H)));

    map.insert(0x28, leak(JumpRelativeOperation(JumpRelativeFlag::Zero)));

    map.insert(0x2C, leak(Inc8Operation(Register::L)));

    map.insert(0x2E, leak(Load8ImmediateOperation(Register::L)));
}

fn load_rank_3_ops(map: &mut OpCodeMap) {
    map.insert(
        0x30,
        leak(JumpRelativeOperation(JumpRelativeFlag::NotCarry)),
    );

    map.insert(0x31, leak(Load16ImmediateOperation(WideRegister::SP)));

    // TODO(berwyn): 0x32 ld (hl-), A <-- what is HL-?

    map.insert(0x33, leak(Inc16Operation(WideRegister::SP)));

    map.insert(0x38, leak(JumpRelativeOperation(JumpRelativeFlag::Carry)));

    map.insert(0x3C, leak(Inc8Operation(Register::A)));

    map.insert(0x3E, leak(Load8ImmediateOperation(Register::A)));
}

fn load_rank_4_ops(map: &mut OpCodeMap) {
    map.insert(
        0x40,
        leak(Load8RegisterCopyOperation(Register::B, Register::B)),
    );

    map.insert(
        0x41,
        leak(Load8RegisterCopyOperation(Register::B, Register::C)),
    );

    map.insert(
        0x42,
        leak(Load8RegisterCopyOperation(Register::B, Register::D)),
    );

    map.insert(
        0x43,
        leak(Load8RegisterCopyOperation(Register::B, Register::E)),
    );

    map.insert(
        0x44,
        leak(Load8RegisterCopyOperation(Register::B, Register::H)),
    );

    map.insert(
        0x45,
        leak(Load8RegisterCopyOperation(Register::B, Register::L)),
    );

    map.insert(
        0x46,
        leak(Load8FromMemoryOperation(Register::B, WideRegister::HL)),
    );

    map.insert(
        0x47,
        leak(Load8RegisterCopyOperation(Register::B, Register::A)),
    );

    map.insert(
        0x48,
        leak(Load8RegisterCopyOperation(Register::C, Register::B)),
    );

    map.insert(
        0x49,
        leak(Load8RegisterCopyOperation(Register::C, Register::C)),
    );

    map.insert(
        0x4A,
        leak(Load8RegisterCopyOperation(Register::C, Register::D)),
    );

    map.insert(
        0x4B,
        leak(Load8RegisterCopyOperation(Register::C, Register::E)),
    );

    map.insert(
        0x4C,
        leak(Load8RegisterCopyOperation(Register::C, Register::H)),
    );

    map.insert(
        0x4D,
        leak(Load8RegisterCopyOperation(Register::C, Register::L)),
    );

    map.insert(
        0x4E,
        leak(Load8FromMemoryOperation(Register::C, WideRegister::HL)),
    );

    map.insert(
        0x4F,
        leak(Load8RegisterCopyOperation(Register::C, Register::A)),
    );
}

fn load_rank_5_ops(map: &mut OpCodeMap) {
    map.insert(
        0x50,
        leak(Load8RegisterCopyOperation(Register::B, Register::B)),
    );

    map.insert(
        0x51,
        leak(Load8RegisterCopyOperation(Register::B, Register::C)),
    );

    map.insert(
        0x52,
        leak(Load8RegisterCopyOperation(Register::B, Register::D)),
    );

    map.insert(
        0x53,
        leak(Load8RegisterCopyOperation(Register::B, Register::E)),
    );

    map.insert(
        0x54,
        leak(Load8RegisterCopyOperation(Register::B, Register::H)),
    );

    map.insert(
        0x55,
        leak(Load8RegisterCopyOperation(Register::B, Register::L)),
    );

    map.insert(
        0x56,
        leak(Load8FromMemoryOperation(Register::B, WideRegister::HL)),
    );

    map.insert(
        0x57,
        leak(Load8RegisterCopyOperation(Register::B, Register::A)),
    );

    map.insert(
        0x58,
        leak(Load8RegisterCopyOperation(Register::C, Register::B)),
    );

    map.insert(
        0x59,
        leak(Load8RegisterCopyOperation(Register::C, Register::C)),
    );

    map.insert(
        0x5A,
        leak(Load8RegisterCopyOperation(Register::C, Register::D)),
    );

    map.insert(
        0x5B,
        leak(Load8RegisterCopyOperation(Register::C, Register::E)),
    );

    map.insert(
        0x5C,
        leak(Load8RegisterCopyOperation(Register::C, Register::H)),
    );

    map.insert(
        0x5D,
        leak(Load8RegisterCopyOperation(Register::C, Register::L)),
    );

    map.insert(
        0x5E,
        leak(Load8FromMemoryOperation(Register::C, WideRegister::HL)),
    );

    map.insert(
        0x5F,
        leak(Load8RegisterCopyOperation(Register::C, Register::A)),
    );
}

fn load_rank_6_ops(map: &mut OpCodeMap) {
    map.insert(
        0x60,
        leak(Load8RegisterCopyOperation(Register::D, Register::B)),
    );

    map.insert(
        0x61,
        leak(Load8RegisterCopyOperation(Register::D, Register::C)),
    );

    map.insert(
        0x62,
        leak(Load8RegisterCopyOperation(Register::D, Register::D)),
    );

    map.insert(
        0x63,
        leak(Load8RegisterCopyOperation(Register::D, Register::E)),
    );

    map.insert(
        0x64,
        leak(Load8RegisterCopyOperation(Register::D, Register::H)),
    );

    map.insert(
        0x65,
        leak(Load8RegisterCopyOperation(Register::D, Register::L)),
    );

    map.insert(
        0x66,
        leak(Load8FromMemoryOperation(Register::D, WideRegister::HL)),
    );

    map.insert(
        0x67,
        leak(Load8RegisterCopyOperation(Register::D, Register::A)),
    );

    map.insert(
        0x68,
        leak(Load8RegisterCopyOperation(Register::E, Register::B)),
    );

    map.insert(
        0x69,
        leak(Load8RegisterCopyOperation(Register::E, Register::C)),
    );

    map.insert(
        0x6A,
        leak(Load8RegisterCopyOperation(Register::E, Register::D)),
    );

    map.insert(
        0x6B,
        leak(Load8RegisterCopyOperation(Register::E, Register::E)),
    );

    map.insert(
        0x6C,
        leak(Load8RegisterCopyOperation(Register::E, Register::H)),
    );

    map.insert(
        0x6D,
        leak(Load8RegisterCopyOperation(Register::E, Register::L)),
    );

    map.insert(
        0x6E,
        leak(Load8FromMemoryOperation(Register::E, WideRegister::HL)),
    );

    map.insert(
        0x6F,
        leak(Load8RegisterCopyOperation(Register::E, Register::A)),
    );
}

fn load_rank_7_ops(map: &mut OpCodeMap) {
    map.insert(
        0x70,
        leak(Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::WideRegister(WideRegister::HL),
            Register::B,
        )),
    );

    map.insert(
        0x71,
        leak(Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::WideRegister(WideRegister::HL),
            Register::C,
        )),
    );

    map.insert(
        0x72,
        leak(Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::WideRegister(WideRegister::HL),
            Register::D,
        )),
    );

    map.insert(
        0x73,
        leak(Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::WideRegister(WideRegister::HL),
            Register::E,
        )),
    );

    map.insert(
        0x74,
        leak(Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::WideRegister(WideRegister::HL),
            Register::H,
        )),
    );

    map.insert(
        0x75,
        leak(Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::WideRegister(WideRegister::HL),
            Register::L,
        )),
    );

    map.insert(0x76, leak(HaltOperation));

    map.insert(
        0x77,
        leak(Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::WideRegister(WideRegister::HL),
            Register::A,
        )),
    );

    map.insert(
        0x78,
        leak(Load8RegisterCopyOperation(Register::A, Register::B)),
    );

    map.insert(
        0x79,
        leak(Load8RegisterCopyOperation(Register::A, Register::C)),
    );

    map.insert(
        0x7A,
        leak(Load8RegisterCopyOperation(Register::A, Register::D)),
    );

    map.insert(
        0x7B,
        leak(Load8RegisterCopyOperation(Register::A, Register::E)),
    );

    map.insert(
        0x7C,
        leak(Load8RegisterCopyOperation(Register::A, Register::H)),
    );

    map.insert(
        0x7D,
        leak(Load8RegisterCopyOperation(Register::A, Register::L)),
    );

    map.insert(
        0x7E,
        leak(Load8FromMemoryOperation(Register::A, WideRegister::HL)),
    );

    map.insert(
        0x7F,
        leak(Load8RegisterCopyOperation(Register::A, Register::A)),
    );
}

fn load_rank_8_ops(map: &mut OpCodeMap) {
    map.insert(0x80, leak(Add8Operation(Register::A, Register::B)));
}

#[allow(non_snake_case)] // `C` is a hex number here, not a letter
fn load_rank_C_ops(map: &mut OpCodeMap) {
    map.insert(0xC1, leak(PopOperation(WideRegister::BC)));

    map.insert(
        0xC2,
        leak(JumpPositionOperation(JumpPositionFlags::NotZero)),
    );

    map.insert(0xC3, leak(JumpPositionOperation(JumpPositionFlags::Nop)));

    map.insert(0xC5, leak(PushOperation(WideRegister::BC)));

    map.insert(0xCA, leak(JumpPositionOperation(JumpPositionFlags::Zero)));
}

#[allow(non_snake_case)] // `D` is a hex number here, not a letter
fn load_rank_D_ops(map: &mut OpCodeMap) {
    map.insert(0xD1, leak(PopOperation(WideRegister::DE)));

    map.insert(
        0xD2,
        leak(JumpPositionOperation(JumpPositionFlags::NotCarry)),
    );

    map.insert(0xD5, leak(PushOperation(WideRegister::DE)));

    map.insert(0xDA, leak(JumpPositionOperation(JumpPositionFlags::Carry)));
}

#[allow(non_snake_case)] // `E` is a hex number here, not a letter
fn load_rank_E_ops(map: &mut OpCodeMap) {
    map.insert(0xE1, leak(PopOperation(WideRegister::DE)));

    map.insert(
        0xE2,
        leak(Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::Register(Register::C),
            Register::A,
        )),
    );

    map.insert(0xE5, leak(PushOperation(WideRegister::HL)));

    map.insert(
        0xE9,
        leak(JumpPositionOperation(JumpPositionFlags::Register)),
    );
}

#[allow(non_snake_case)] // 'F' is a hex number here, not a letter
fn load_rank_F_ops(map: &mut OpCodeMap) {
    map.insert(0xF1, leak(PopOperation(WideRegister::HL)));

    map.insert(0xF3, leak(DisableInterruptsOperation));

    map.insert(0xF5, leak(PushOperation(WideRegister::AF)));

    map.insert(0xFB, leak(EnableInterruptsOperation));
}
