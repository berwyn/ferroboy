use crate::{
    operations::*,
    system::{Register, WideRegister},
};

pub struct StaticOpcodeMap;

pub static OPCODES: StaticOpcodeMap = StaticOpcodeMap;

impl StaticOpcodeMap {
    pub fn get(&self, code: &u8) -> Option<&&'static dyn Operation> {
        INTERNAL_OPCODES
            .binary_search_by_key(&code, |(code, _)| code)
            .map(|idx| &INTERNAL_OPCODES[idx].1)
            .ok()
    }
}

/// A compile-time map of opcodes to their Operations.
static INTERNAL_OPCODES: &[(u8, &'static dyn Operation)] = &[
    ////////////////////
    // Rank 0 opcodes
    (0x00, &NopOperation),
    (0x01, &Load16ImmediateOperation(WideRegister::Bc)),
    (
        0x02,
        &Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::WideRegister(WideRegister::Bc),
            Register::A,
        ),
    ),
    (0x03, &Inc16Operation(WideRegister::Bc)),
    (0x04, &Inc8Operation(Register::B)),
    (0x05, &Dec8Operation(Register::B)),
    (0x06, &Load8ImmediateOperation(Register::B)),
    (0x07, &RlcaOperation),
    (0x09, &Add16Operation(WideRegister::Bc)),
    (
        0x0A,
        &Load8FromMemoryOperation(Register::A, WideRegister::Bc),
    ),
    (0x0B, &Dec16Operation(WideRegister::Bc)),
    (0x0C, &Inc8Operation(Register::C)),
    (0x0D, &Dec8Operation(Register::C)),
    (0x0E, &Load8ImmediateOperation(Register::C)),
    (0x0F, &RrcaOperation),
    ////////////////////
    // Rank 1 opcodes
    (0x10, &StopOperation),
    (0x11, &Load16ImmediateOperation(WideRegister::De)),
    (
        0x12,
        &Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::WideRegister(WideRegister::De),
            Register::A,
        ),
    ),
    (0x13, &Inc16Operation(WideRegister::De)),
    (0x14, &Inc8Operation(Register::D)),
    (0x15, &Dec8Operation(Register::D)),
    (0x16, &Load8ImmediateOperation(Register::D)),
    (0x18, &JumpRelativeOperation(JumpRelativeFlag::Nop)),
    (0x19, &Add16Operation(WideRegister::De)),
    (
        0x1A,
        &Load8FromMemoryOperation(Register::A, WideRegister::De),
    ),
    (0x1B, &Dec16Operation(WideRegister::De)),
    (0x1C, &Inc8Operation(Register::E)),
    (0x1D, &Dec8Operation(Register::E)),
    (0x1E, &Load8ImmediateOperation(Register::E)),
    ////////////////////
    // Rank 2 opcodes
    (0x20, &JumpRelativeOperation(JumpRelativeFlag::NotZero)),
    (0x21, &Load16ImmediateOperation(WideRegister::Hl)),
    (0x22, &Load8AbsoluteOperation(Load8AbsoluteTarget::HLPositive)),
    (0x23, &Inc16Operation(WideRegister::Hl)),
    (0x24, &Inc8Operation(Register::H)),
    (0x25, &Dec8Operation(Register::H)),
    (0x26, &Load8ImmediateOperation(Register::H)),
    (0x28, &JumpRelativeOperation(JumpRelativeFlag::Zero)),
    (0x29, &Add16Operation(WideRegister::Hl)),
    (0x2B, &Dec16Operation(WideRegister::Hl)),
    (0x2C, &Inc8Operation(Register::L)),
    (0x2D, &Dec8Operation(Register::L)),
    (0x2E, &Load8ImmediateOperation(Register::L)),
    ////////////////////
    // Rank 3 opcodes
    (0x30, &JumpRelativeOperation(JumpRelativeFlag::NotCarry)),
    (0x31, &Load16ImmediateOperation(WideRegister::Sp)),
    (0x32, &Load8AbsoluteOperation(Load8AbsoluteTarget::HLNegative)),
    (0x33, &Inc16Operation(WideRegister::Sp)),
    (0x38, &JumpRelativeOperation(JumpRelativeFlag::Carry)),
    (0x39, &Add16Operation(WideRegister::Sp)),
    (0x3B, &Dec16Operation(WideRegister::Sp)),
    (0x3C, &Inc8Operation(Register::A)),
    (0x3D, &Dec8Operation(Register::A)),
    (0x3E, &Load8ImmediateOperation(Register::A)),
    ////////////////////
    // Rank 4 opcodes
    (0x40, &Load8RegisterCopyOperation(Register::B, Register::B)),
    (0x41, &Load8RegisterCopyOperation(Register::B, Register::C)),
    (0x42, &Load8RegisterCopyOperation(Register::B, Register::D)),
    (0x43, &Load8RegisterCopyOperation(Register::B, Register::E)),
    (0x44, &Load8RegisterCopyOperation(Register::B, Register::H)),
    (0x45, &Load8RegisterCopyOperation(Register::B, Register::L)),
    (
        0x46,
        &Load8FromMemoryOperation(Register::B, WideRegister::Hl),
    ),
    (0x47, &Load8RegisterCopyOperation(Register::B, Register::A)),
    (0x48, &Load8RegisterCopyOperation(Register::C, Register::B)),
    (0x49, &Load8RegisterCopyOperation(Register::C, Register::C)),
    (0x4A, &Load8RegisterCopyOperation(Register::C, Register::D)),
    (0x4B, &Load8RegisterCopyOperation(Register::C, Register::E)),
    (0x4C, &Load8RegisterCopyOperation(Register::C, Register::H)),
    (0x4D, &Load8RegisterCopyOperation(Register::C, Register::L)),
    (
        0x4E,
        &Load8FromMemoryOperation(Register::C, WideRegister::Hl),
    ),
    (0x4F, &Load8RegisterCopyOperation(Register::C, Register::A)),
    ////////////////////
    // Rank 5 opcodes
    (0x50, &Load8RegisterCopyOperation(Register::D, Register::B)),
    (0x51, &Load8RegisterCopyOperation(Register::D, Register::C)),
    (0x52, &Load8RegisterCopyOperation(Register::D, Register::D)),
    (0x53, &Load8RegisterCopyOperation(Register::D, Register::E)),
    (0x54, &Load8RegisterCopyOperation(Register::D, Register::H)),
    (0x55, &Load8RegisterCopyOperation(Register::D, Register::L)),
    (
        0x56,
        &Load8FromMemoryOperation(Register::D, WideRegister::Hl),
    ),
    (0x57, &Load8RegisterCopyOperation(Register::D, Register::A)),
    (0x58, &Load8RegisterCopyOperation(Register::E, Register::B)),
    (0x59, &Load8RegisterCopyOperation(Register::E, Register::C)),
    (0x5A, &Load8RegisterCopyOperation(Register::E, Register::D)),
    (0x5B, &Load8RegisterCopyOperation(Register::E, Register::E)),
    (0x5C, &Load8RegisterCopyOperation(Register::E, Register::H)),
    (0x5D, &Load8RegisterCopyOperation(Register::E, Register::L)),
    (
        0x5E,
        &Load8FromMemoryOperation(Register::E, WideRegister::Hl),
    ),
    (0x5F, &Load8RegisterCopyOperation(Register::E, Register::A)),
    ////////////////////
    // Rank 6 opcodes
    (0x60, &Load8RegisterCopyOperation(Register::H, Register::B)),
    (0x61, &Load8RegisterCopyOperation(Register::H, Register::C)),
    (0x62, &Load8RegisterCopyOperation(Register::H, Register::D)),
    (0x63, &Load8RegisterCopyOperation(Register::H, Register::E)),
    (0x64, &Load8RegisterCopyOperation(Register::H, Register::H)),
    (0x65, &Load8RegisterCopyOperation(Register::H, Register::L)),
    (
        0x66,
        &Load8FromMemoryOperation(Register::H, WideRegister::Hl),
    ),
    (0x67, &Load8RegisterCopyOperation(Register::H, Register::A)),
    (0x68, &Load8RegisterCopyOperation(Register::L, Register::B)),
    (0x69, &Load8RegisterCopyOperation(Register::L, Register::C)),
    (0x6A, &Load8RegisterCopyOperation(Register::L, Register::D)),
    (0x6B, &Load8RegisterCopyOperation(Register::L, Register::E)),
    (0x6C, &Load8RegisterCopyOperation(Register::L, Register::H)),
    (0x6D, &Load8RegisterCopyOperation(Register::L, Register::L)),
    (
        0x6E,
        &Load8FromMemoryOperation(Register::L, WideRegister::Hl),
    ),
    (0x6F, &Load8RegisterCopyOperation(Register::L, Register::A)),
    ////////////////////
    // Rank 7 opcodes
    (
        0x70,
        &Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::WideRegister(WideRegister::Hl),
            Register::B,
        ),
    ),
    (
        0x71,
        &Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::WideRegister(WideRegister::Hl),
            Register::C,
        ),
    ),
    (
        0x72,
        &Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::WideRegister(WideRegister::Hl),
            Register::D,
        ),
    ),
    (
        0x73,
        &Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::WideRegister(WideRegister::Hl),
            Register::E,
        ),
    ),
    (
        0x74,
        &Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::WideRegister(WideRegister::Hl),
            Register::H,
        ),
    ),
    (
        0x75,
        &Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::WideRegister(WideRegister::Hl),
            Register::L,
        ),
    ),
    (0x76, &HaltOperation),
    (
        0x77,
        &Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::WideRegister(WideRegister::Hl),
            Register::A,
        ),
    ),
    (0x78, &Load8RegisterCopyOperation(Register::A, Register::B)),
    (0x79, &Load8RegisterCopyOperation(Register::A, Register::C)),
    (0x7A, &Load8RegisterCopyOperation(Register::A, Register::D)),
    (0x7B, &Load8RegisterCopyOperation(Register::A, Register::E)),
    (0x7C, &Load8RegisterCopyOperation(Register::A, Register::H)),
    (0x7D, &Load8RegisterCopyOperation(Register::A, Register::L)),
    (
        0x7E,
        &Load8FromMemoryOperation(Register::A, WideRegister::Hl),
    ),
    (0x7F, &Load8RegisterCopyOperation(Register::A, Register::A)),
    ////////////////////
    // Rank 8 opcodes
    (0x80, &Add8Operation(Register::A, Register::B)),
    (0x81, &Add8Operation(Register::A, Register::C)),
    (0x82, &Add8Operation(Register::A, Register::D)),
    (0x83, &Add8Operation(Register::A, Register::E)),
    (0x85, &Add8Operation(Register::A, Register::H)),
    (0x86, &Add8Operation(Register::A, Register::L)),
    (0x88, &Add8Operation(Register::A, Register::A)),
    ////////////////////
    // Rank 9 opcodes
    (0x90, &SubOperation(SubTarget::Register(Register::B))),
    (0x91, &SubOperation(SubTarget::Register(Register::C))),
    (0x92, &SubOperation(SubTarget::Register(Register::D))),
    (0x93, &SubOperation(SubTarget::Register(Register::E))),
    (0x94, &SubOperation(SubTarget::Register(Register::H))),
    (0x95, &SubOperation(SubTarget::Register(Register::L))),
    (0x96, &SubOperation(SubTarget::Address)),
    (0x97, &SubOperation(SubTarget::Register(Register::A))),
    ////////////////////
    // Rank A opcodes
    (0xA0, &AndOperation(AndTarget::Register(Register::B))),
    (0xA1, &AndOperation(AndTarget::Register(Register::C))),
    (0xA2, &AndOperation(AndTarget::Register(Register::D))),
    (0xA3, &AndOperation(AndTarget::Register(Register::E))),
    (0xA4, &AndOperation(AndTarget::Register(Register::H))),
    (0xA5, &AndOperation(AndTarget::Register(Register::L))),
    (0xA6, &AndOperation(AndTarget::Address)),
    (0xA7, &AndOperation(AndTarget::Register(Register::A))),
    (0xA8, &XorOperation(XorTarget::Register(Register::B))),
    (0xA9, &XorOperation(XorTarget::Register(Register::C))),
    (0xAA, &XorOperation(XorTarget::Register(Register::D))),
    (0xAB, &XorOperation(XorTarget::Register(Register::E))),
    (0xAC, &XorOperation(XorTarget::Register(Register::H))),
    (0xAD, &XorOperation(XorTarget::Register(Register::L))),
    (0xAE, &XorOperation(XorTarget::Address)),
    (0xAF, &XorOperation(XorTarget::Register(Register::A))),
    ////////////////////
    // Rank B opcodes
    (0xB0, &OrOperation(OrTarget::Register(Register::B))),
    (0xB1, &OrOperation(OrTarget::Register(Register::C))),
    (0xB2, &OrOperation(OrTarget::Register(Register::D))),
    (0xB3, &OrOperation(OrTarget::Register(Register::E))),
    (0xB4, &OrOperation(OrTarget::Register(Register::H))),
    (0xB5, &OrOperation(OrTarget::Register(Register::L))),
    (0xB6, &OrOperation(OrTarget::Address)),
    (0xB7, &OrOperation(OrTarget::Register(Register::A))),
    (0xB8, &CpOperation(CpTarget::Register(Register::B))),
    (0xB9, &CpOperation(CpTarget::Register(Register::C))),
    (0xBA, &CpOperation(CpTarget::Register(Register::D))),
    (0xBB, &CpOperation(CpTarget::Register(Register::E))),
    (0xBC, &CpOperation(CpTarget::Register(Register::H))),
    (0xBD, &CpOperation(CpTarget::Register(Register::L))),
    (0xBE, &CpOperation(CpTarget::Address)),
    (0xBF, &CpOperation(CpTarget::Register(Register::A))),
    ////////////////////
    // Rank C opcodes
    (0xC0, &RetOperation(Some(RetCondition::NotZero))),
    (0xC1, &PopOperation(WideRegister::Bc)),
    (0xC2, &JumpPositionOperation(JumpPositionFlags::NotZero)),
    (0xC3, &JumpPositionOperation(JumpPositionFlags::Nop)),
    (0xC4, &CallOperation(Some(CallCondition::NotZero))),
    (0xC5, &PushOperation(WideRegister::Bc)),
    (0xC7, &RstOperation(0x00)),
    (0xC8, &RetOperation(Some(RetCondition::Zero))),
    (0xC9, &RetOperation(None)),
    (0xCA, &JumpPositionOperation(JumpPositionFlags::Zero)),
    (0xCB, &PrefixOperation),
    (0xCC, &CallOperation(Some(CallCondition::Zero))),
    (0xCD, &CallOperation(None)),
    (0xCF, &RstOperation(0x08)),
    ////////////////////
    // Rank D opcodes
    (0xD0, &RetOperation(Some(RetCondition::NotCarry))),
    (0xD1, &PopOperation(WideRegister::De)),
    (0xD2, &JumpPositionOperation(JumpPositionFlags::NotCarry)),
    (0xD4, &CallOperation(Some(CallCondition::NotCarry))),
    (0xD5, &PushOperation(WideRegister::De)),
    (0xD6, &SubOperation(SubTarget::Immediate)),
    (0xD7, &RstOperation(0x10)),
    (0xD8, &RetOperation(Some(RetCondition::Carry))),
    (0xDA, &JumpPositionOperation(JumpPositionFlags::Carry)),
    (0xDC, &CallOperation(Some(CallCondition::Carry))),
    (0xDF, &RstOperation(0x18)),
    ////////////////////
    // Rank E opcodes
    (0xE1, &PopOperation(WideRegister::De)),
    (
        0xE2,
        &Load8RegisterToMemoryOperation(
            Load8RegisterToMemoryTarget::Register(Register::C),
            Register::A,
        ),
    ),
    (0xE5, &PushOperation(WideRegister::Hl)),
    (0xE6, &AndOperation(AndTarget::Immediate)),
    (0xE7, &RstOperation(0x20)),
    (0xE9, &JumpPositionOperation(JumpPositionFlags::Register)),
    (0xEE, &XorOperation(XorTarget::Immediate)),
    (0xEF, &RstOperation(0x28)),
    ////////////////////
    // Rank F opcodes
    (0xF1, &PopOperation(WideRegister::Hl)),
    (0xF3, &DisableInterruptsOperation),
    (0xF5, &PushOperation(WideRegister::Af)),
    (0xF6, &OrOperation(OrTarget::Immediate)),
    (0xF7, &RstOperation(0x30)),
    (0xFB, &EnableInterruptsOperation),
    (0xFE, &CpOperation(CpTarget::Immediate)),
    (0xFF, &RstOperation(0x38)),
];
