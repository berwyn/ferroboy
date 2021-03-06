use std::collections::BTreeMap;

use once_cell::sync::Lazy;

use crate::{
    operations::*,
    system::{Register, WideRegister},
};

pub(crate) type OpCodeMap = BTreeMap<u8, &'static dyn Operation>;

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
    load_rank_9_ops(&mut map);
    load_rank_A_ops(&mut map);
    load_rank_B_ops(&mut map);
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

    map.insert(0x01, leak(Load16ImmediateOperation(WideRegister::Bc)));

    map.insert(
        0x02,
        leak(Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::WideRegister(WideRegister::Bc),
            Register::A,
        )),
    );

    map.insert(0x03, leak(Inc16Operation(WideRegister::Bc)));

    map.insert(0x04, leak(Inc8Operation(Register::B)));

    map.insert(0x05, leak(Dec8Operation(Register::B)));

    map.insert(0x06, leak(Load8ImmediateOperation(Register::B)));

    map.insert(0x07, leak(RlcaOperation));

    map.insert(0x09, leak(Add16Operation(WideRegister::Bc)));

    map.insert(
        0x0A,
        leak(Load8FromMemoryOperation(Register::A, WideRegister::Bc)),
    );

    map.insert(0x0B, leak(Dec16Operation(WideRegister::Bc)));

    map.insert(0x0C, leak(Inc8Operation(Register::C)));

    map.insert(0x0D, leak(Dec8Operation(Register::C)));

    map.insert(0x0E, leak(Load8ImmediateOperation(Register::C)));

    map.insert(0x0F, leak(RrcaOperation));
}

fn load_rank_1_ops(map: &mut OpCodeMap) {
    map.insert(0x10, leak(StopOperation));

    map.insert(0x11, leak(Load16ImmediateOperation(WideRegister::De)));

    map.insert(
        0x12,
        leak(Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::WideRegister(WideRegister::De),
            Register::A,
        )),
    );

    map.insert(0x13, leak(Inc16Operation(WideRegister::De)));

    map.insert(0x14, leak(Inc8Operation(Register::D)));

    map.insert(0x15, leak(Dec8Operation(Register::D)));

    map.insert(0x16, leak(Load8ImmediateOperation(Register::D)));

    map.insert(0x18, leak(JumpRelativeOperation(JumpRelativeFlag::Nop)));

    map.insert(0x19, leak(Add16Operation(WideRegister::De)));

    map.insert(
        0x1A,
        leak(Load8FromMemoryOperation(Register::A, WideRegister::De)),
    );

    map.insert(0x1B, leak(Dec16Operation(WideRegister::De)));

    map.insert(0x1C, leak(Inc8Operation(Register::E)));

    map.insert(0x1D, leak(Dec8Operation(Register::E)));

    map.insert(0x1E, leak(Load8ImmediateOperation(Register::E)));
}

fn load_rank_2_ops(map: &mut OpCodeMap) {
    map.insert(0x20, leak(JumpRelativeOperation(JumpRelativeFlag::NotZero)));

    map.insert(0x21, leak(Load16ImmediateOperation(WideRegister::Hl)));

    // TODO(berwyn): 0x22 ld (hl+), A <-- what is HL+?

    map.insert(0x23, leak(Inc16Operation(WideRegister::Hl)));

    map.insert(0x24, leak(Inc8Operation(Register::H)));

    map.insert(0x25, leak(Dec8Operation(Register::H)));

    map.insert(0x26, leak(Load8ImmediateOperation(Register::H)));

    map.insert(0x28, leak(JumpRelativeOperation(JumpRelativeFlag::Zero)));

    map.insert(0x29, leak(Add16Operation(WideRegister::Hl)));

    map.insert(0x2B, leak(Dec16Operation(WideRegister::Hl)));

    map.insert(0x2C, leak(Inc8Operation(Register::L)));

    map.insert(0x2D, leak(Dec8Operation(Register::L)));

    map.insert(0x2E, leak(Load8ImmediateOperation(Register::L)));
}

fn load_rank_3_ops(map: &mut OpCodeMap) {
    map.insert(
        0x30,
        leak(JumpRelativeOperation(JumpRelativeFlag::NotCarry)),
    );

    map.insert(0x31, leak(Load16ImmediateOperation(WideRegister::Sp)));

    // TODO(berwyn): 0x32 ld (hl-), A <-- what is HL-?

    map.insert(0x33, leak(Inc16Operation(WideRegister::Sp)));

    map.insert(0x38, leak(JumpRelativeOperation(JumpRelativeFlag::Carry)));

    map.insert(0x39, leak(Add16Operation(WideRegister::Sp)));

    map.insert(0x3B, leak(Dec16Operation(WideRegister::Sp)));

    map.insert(0x3C, leak(Inc8Operation(Register::A)));

    map.insert(0x3D, leak(Dec8Operation(Register::A)));

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
        leak(Load8FromMemoryOperation(Register::B, WideRegister::Hl)),
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
        leak(Load8FromMemoryOperation(Register::C, WideRegister::Hl)),
    );

    map.insert(
        0x4F,
        leak(Load8RegisterCopyOperation(Register::C, Register::A)),
    );
}

fn load_rank_5_ops(map: &mut OpCodeMap) {
    map.insert(
        0x50,
        leak(Load8RegisterCopyOperation(Register::D, Register::B)),
    );

    map.insert(
        0x51,
        leak(Load8RegisterCopyOperation(Register::D, Register::C)),
    );

    map.insert(
        0x52,
        leak(Load8RegisterCopyOperation(Register::D, Register::D)),
    );

    map.insert(
        0x53,
        leak(Load8RegisterCopyOperation(Register::D, Register::E)),
    );

    map.insert(
        0x54,
        leak(Load8RegisterCopyOperation(Register::D, Register::H)),
    );

    map.insert(
        0x55,
        leak(Load8RegisterCopyOperation(Register::D, Register::L)),
    );

    map.insert(
        0x56,
        leak(Load8FromMemoryOperation(Register::D, WideRegister::Hl)),
    );

    map.insert(
        0x57,
        leak(Load8RegisterCopyOperation(Register::D, Register::A)),
    );

    map.insert(
        0x58,
        leak(Load8RegisterCopyOperation(Register::E, Register::B)),
    );

    map.insert(
        0x59,
        leak(Load8RegisterCopyOperation(Register::E, Register::C)),
    );

    map.insert(
        0x5A,
        leak(Load8RegisterCopyOperation(Register::E, Register::D)),
    );

    map.insert(
        0x5B,
        leak(Load8RegisterCopyOperation(Register::E, Register::E)),
    );

    map.insert(
        0x5C,
        leak(Load8RegisterCopyOperation(Register::E, Register::H)),
    );

    map.insert(
        0x5D,
        leak(Load8RegisterCopyOperation(Register::E, Register::L)),
    );

    map.insert(
        0x5E,
        leak(Load8FromMemoryOperation(Register::E, WideRegister::Hl)),
    );

    map.insert(
        0x5F,
        leak(Load8RegisterCopyOperation(Register::E, Register::A)),
    );
}

fn load_rank_6_ops(map: &mut OpCodeMap) {
    map.insert(
        0x60,
        leak(Load8RegisterCopyOperation(Register::H, Register::B)),
    );

    map.insert(
        0x61,
        leak(Load8RegisterCopyOperation(Register::H, Register::C)),
    );

    map.insert(
        0x62,
        leak(Load8RegisterCopyOperation(Register::H, Register::D)),
    );

    map.insert(
        0x63,
        leak(Load8RegisterCopyOperation(Register::H, Register::E)),
    );

    map.insert(
        0x64,
        leak(Load8RegisterCopyOperation(Register::H, Register::H)),
    );

    map.insert(
        0x65,
        leak(Load8RegisterCopyOperation(Register::H, Register::L)),
    );

    map.insert(
        0x66,
        leak(Load8FromMemoryOperation(Register::H, WideRegister::Hl)),
    );

    map.insert(
        0x67,
        leak(Load8RegisterCopyOperation(Register::H, Register::A)),
    );

    map.insert(
        0x68,
        leak(Load8RegisterCopyOperation(Register::L, Register::B)),
    );

    map.insert(
        0x69,
        leak(Load8RegisterCopyOperation(Register::L, Register::C)),
    );

    map.insert(
        0x6A,
        leak(Load8RegisterCopyOperation(Register::L, Register::D)),
    );

    map.insert(
        0x6B,
        leak(Load8RegisterCopyOperation(Register::L, Register::E)),
    );

    map.insert(
        0x6C,
        leak(Load8RegisterCopyOperation(Register::L, Register::H)),
    );

    map.insert(
        0x6D,
        leak(Load8RegisterCopyOperation(Register::L, Register::L)),
    );

    map.insert(
        0x6E,
        leak(Load8FromMemoryOperation(Register::L, WideRegister::Hl)),
    );

    map.insert(
        0x6F,
        leak(Load8RegisterCopyOperation(Register::L, Register::A)),
    );
}

fn load_rank_7_ops(map: &mut OpCodeMap) {
    map.insert(
        0x70,
        leak(Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::WideRegister(WideRegister::Hl),
            Register::B,
        )),
    );

    map.insert(
        0x71,
        leak(Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::WideRegister(WideRegister::Hl),
            Register::C,
        )),
    );

    map.insert(
        0x72,
        leak(Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::WideRegister(WideRegister::Hl),
            Register::D,
        )),
    );

    map.insert(
        0x73,
        leak(Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::WideRegister(WideRegister::Hl),
            Register::E,
        )),
    );

    map.insert(
        0x74,
        leak(Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::WideRegister(WideRegister::Hl),
            Register::H,
        )),
    );

    map.insert(
        0x75,
        leak(Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::WideRegister(WideRegister::Hl),
            Register::L,
        )),
    );

    map.insert(0x76, leak(HaltOperation));

    map.insert(
        0x77,
        leak(Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::WideRegister(WideRegister::Hl),
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
        leak(Load8FromMemoryOperation(Register::A, WideRegister::Hl)),
    );

    map.insert(
        0x7F,
        leak(Load8RegisterCopyOperation(Register::A, Register::A)),
    );
}

fn load_rank_8_ops(map: &mut OpCodeMap) {
    map.insert(0x80, leak(Add8Operation(Register::A, Register::B)));
    map.insert(0x81, leak(Add8Operation(Register::A, Register::C)));
    map.insert(0x82, leak(Add8Operation(Register::A, Register::D)));
    map.insert(0x83, leak(Add8Operation(Register::A, Register::E)));
    map.insert(0x85, leak(Add8Operation(Register::A, Register::H)));
    map.insert(0x86, leak(Add8Operation(Register::A, Register::L)));
    map.insert(0x88, leak(Add8Operation(Register::A, Register::A)));
}

fn load_rank_9_ops(map: &mut OpCodeMap) {
    map.insert(0x90, leak(SubOperation(SubTarget::Register(Register::B))));
    map.insert(0x91, leak(SubOperation(SubTarget::Register(Register::C))));
    map.insert(0x92, leak(SubOperation(SubTarget::Register(Register::D))));
    map.insert(0x93, leak(SubOperation(SubTarget::Register(Register::E))));
    map.insert(0x94, leak(SubOperation(SubTarget::Register(Register::H))));
    map.insert(0x95, leak(SubOperation(SubTarget::Register(Register::L))));
    map.insert(0x96, leak(SubOperation(SubTarget::Address)));
    map.insert(0x97, leak(SubOperation(SubTarget::Register(Register::A))));
}

#[allow(non_snake_case)] // `A` is a hex number here, not a letter
fn load_rank_A_ops(map: &mut OpCodeMap) {
    map.insert(0xA0, leak(AndOperation(AndTarget::Register(Register::B))));
    map.insert(0xA1, leak(AndOperation(AndTarget::Register(Register::C))));
    map.insert(0xA2, leak(AndOperation(AndTarget::Register(Register::D))));
    map.insert(0xA3, leak(AndOperation(AndTarget::Register(Register::E))));
    map.insert(0xA4, leak(AndOperation(AndTarget::Register(Register::H))));
    map.insert(0xA5, leak(AndOperation(AndTarget::Register(Register::L))));
    map.insert(0xA6, leak(AndOperation(AndTarget::Address)));
    map.insert(0xA7, leak(AndOperation(AndTarget::Register(Register::A))));
    map.insert(0xA8, leak(XorOperation(XorTarget::Register(Register::B))));
    map.insert(0xA9, leak(XorOperation(XorTarget::Register(Register::C))));
    map.insert(0xAA, leak(XorOperation(XorTarget::Register(Register::D))));
    map.insert(0xAB, leak(XorOperation(XorTarget::Register(Register::E))));
    map.insert(0xAC, leak(XorOperation(XorTarget::Register(Register::H))));
    map.insert(0xAD, leak(XorOperation(XorTarget::Register(Register::L))));
    map.insert(0xAE, leak(XorOperation(XorTarget::Address)));
    map.insert(0xAF, leak(XorOperation(XorTarget::Register(Register::A))));
}

#[allow(non_snake_case)] // `B` is a hex number here, not a letter
fn load_rank_B_ops(map: &mut OpCodeMap) {
    map.insert(0xB0, leak(OrOperation(OrTarget::Register(Register::B))));
    map.insert(0xB1, leak(OrOperation(OrTarget::Register(Register::C))));
    map.insert(0xB2, leak(OrOperation(OrTarget::Register(Register::D))));
    map.insert(0xB3, leak(OrOperation(OrTarget::Register(Register::E))));
    map.insert(0xB4, leak(OrOperation(OrTarget::Register(Register::H))));
    map.insert(0xB5, leak(OrOperation(OrTarget::Register(Register::L))));
    map.insert(0xB6, leak(OrOperation(OrTarget::Address)));
    map.insert(0xB7, leak(OrOperation(OrTarget::Register(Register::A))));
    map.insert(0xB8, leak(CpOperation(CpTarget::Register(Register::B))));
    map.insert(0xB9, leak(CpOperation(CpTarget::Register(Register::C))));
    map.insert(0xBA, leak(CpOperation(CpTarget::Register(Register::D))));
    map.insert(0xBB, leak(CpOperation(CpTarget::Register(Register::E))));
    map.insert(0xBC, leak(CpOperation(CpTarget::Register(Register::H))));
    map.insert(0xBD, leak(CpOperation(CpTarget::Register(Register::L))));
    map.insert(0xBE, leak(CpOperation(CpTarget::Address)));
    map.insert(0xBF, leak(CpOperation(CpTarget::Register(Register::A))));
}

#[allow(non_snake_case)] // `C` is a hex number here, not a letter
fn load_rank_C_ops(map: &mut OpCodeMap) {
    map.insert(0xC0, leak(RetOperation(Some(RetCondition::NotZero))));
    map.insert(0xC1, leak(PopOperation(WideRegister::Bc)));

    map.insert(
        0xC2,
        leak(JumpPositionOperation(JumpPositionFlags::NotZero)),
    );

    map.insert(0xC3, leak(JumpPositionOperation(JumpPositionFlags::Nop)));
    map.insert(0xC4, leak(CallOperation(Some(CallCondition::NotZero))));
    map.insert(0xC5, leak(PushOperation(WideRegister::Bc)));

    map.insert(0xC7, leak(RstOperation(0x00)));
    map.insert(0xC8, leak(RetOperation(Some(RetCondition::Zero))));
    map.insert(0xC9, leak(RetOperation(None)));
    map.insert(0xCA, leak(JumpPositionOperation(JumpPositionFlags::Zero)));
    map.insert(0xCB, leak(PrefixOperation));
    map.insert(0xCC, leak(CallOperation(Some(CallCondition::Zero))));
    map.insert(0xCD, leak(CallOperation(None)));

    map.insert(0xCF, leak(RstOperation(0x08)));
}

#[allow(non_snake_case)] // `D` is a hex number here, not a letter
fn load_rank_D_ops(map: &mut OpCodeMap) {
    map.insert(0xD0, leak(RetOperation(Some(RetCondition::NotCarry))));
    map.insert(0xD1, leak(PopOperation(WideRegister::De)));

    map.insert(
        0xD2,
        leak(JumpPositionOperation(JumpPositionFlags::NotCarry)),
    );

    map.insert(0xD4, leak(CallOperation(Some(CallCondition::NotCarry))));
    map.insert(0xD5, leak(PushOperation(WideRegister::De)));
    map.insert(0xD6, leak(SubOperation(SubTarget::Immediate)));
    map.insert(0xD7, leak(RstOperation(0x10)));
    map.insert(0xD8, leak(RetOperation(Some(RetCondition::Carry))));

    map.insert(0xDA, leak(JumpPositionOperation(JumpPositionFlags::Carry)));
    map.insert(0xDC, leak(CallOperation(Some(CallCondition::Carry))));

    map.insert(0xDF, leak(RstOperation(0x18)));
}

#[allow(non_snake_case)] // `E` is a hex number here, not a letter
fn load_rank_E_ops(map: &mut OpCodeMap) {
    map.insert(0xE1, leak(PopOperation(WideRegister::De)));

    map.insert(
        0xE2,
        leak(Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::Register(Register::C),
            Register::A,
        )),
    );

    map.insert(0xE5, leak(PushOperation(WideRegister::Hl)));

    map.insert(0xE6, leak(AndOperation(AndTarget::Immediate)));
    map.insert(0xE7, leak(RstOperation(0x20)));

    map.insert(
        0xE9,
        leak(JumpPositionOperation(JumpPositionFlags::Register)),
    );

    map.insert(0xEE, leak(XorOperation(XorTarget::Immediate)));
    map.insert(0xEF, leak(RstOperation(0x28)));
}

#[allow(non_snake_case)] // 'F' is a hex number here, not a letter
fn load_rank_F_ops(map: &mut OpCodeMap) {
    map.insert(0xF1, leak(PopOperation(WideRegister::Hl)));

    map.insert(0xF3, leak(DisableInterruptsOperation));

    map.insert(0xF5, leak(PushOperation(WideRegister::Af)));
    map.insert(0xF6, leak(OrOperation(OrTarget::Immediate)));
    map.insert(0xF7, leak(RstOperation(0x30)));

    map.insert(0xFB, leak(EnableInterruptsOperation));

    map.insert(0xFE, leak(CpOperation(CpTarget::Immediate)));
    map.insert(0xFF, leak(RstOperation(0x38)));
}
