use thiserror::Error;

use crate::system::{Register, WideRegister};

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    CartridgeLoad(#[from] CartridgeLoadError),

    #[error(transparent)]
    Disassembly(#[from] DisassemblyError),

    #[error(transparent)]
    Operation(#[from] OperationError),

    #[error("The state isn't ready yet")]
    StateNotReady,
    #[error("The address {0} isn't valid")]
    AddressOutOfRange(u32),
    #[error("'{0:0<2X}' isn't a valid opcode")]
    InvalidOperation(u8),
}

#[derive(Error, Debug)]
pub enum CartridgeLoadError {
    #[error("No source was provided")]
    NoSourceSet,
    #[error(transparent)]
    FileSystemError(#[from] std::io::Error),
    #[error("The header checksum isn't valid")]
    ChecksumFail,
    #[error("The header title isn't valid")]
    InvalidTitle(#[from] std::string::FromUtf8Error),
    #[error("'{0}' isn't a valid number of banks")]
    InvalidBankCount(u8),
    #[error("'{0} isn't a valid number of RAM banks")]
    InvalidRamSize(u8),
    #[error("The cartridge mapper isn't supported")]
    InvalidMapper,
}

#[derive(Error, Debug)]
pub enum DisassemblyError {
    #[error("No command provided")]
    EmptyCommand,
}

#[derive(Error, Debug)]
pub enum OperationError {
    #[error("{0} isn't a valid target for this operation")]
    InvalidRegister(Register),
    #[error("{0} isn't a valid target for this operation")]
    InvalidWideRegister(WideRegister),
}
