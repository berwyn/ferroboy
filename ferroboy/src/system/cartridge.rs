use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::{
    assembly::{AssemblyInstruction, AssemblyInstructionStream},
    error::CartridgeLoadError,
    system::{Config, MMU},
};

const CARTRIDGE_HEADER: [u8; 48] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

/// An enum to express what memory mapper a cartridge is using.
///
/// This does include the unlicensed memory mappers, however that doesn't mean that
/// these memory mappers will be fully supported. They are included the sake of
/// completeness.
#[repr(u8)]
#[derive(Debug)]
pub enum CartridgeType {
    RomOnly = 0x00,

    MBC1 = 0x01,
    MBC1WithRAM = 0x02,
    MBC1WithRAMAndBattery = 0x03,

    MBC2 = 0x05,
    MBC2WithBattery = 0x06,

    ROMAndRAM = 0x08,
    ROMAndRAMAndBattery = 0x09,

    MMM01 = 0x0B,
    MMM01WithRAM = 0x0C,
    MMM01WithRAMAndBattery = 0x0D,

    MBC3WithTimerAndBattery = 0x0F,
    MBC3WithTimerAndBatteryAndRAM = 0x10,
    MBC3 = 0x11,
    MBC3WithRAM = 0x12,
    MBC3WithRAMAndBattery = 0x13,

    MBC4 = 0x15,
    MBC4WithRAM = 0x16,
    MBC4WithRAMAndBattery = 0x17,

    MBC5 = 0x19,
    MBC5WithRAM = 0x1A,
    MBC5WithRAMAndBattery = 0x1B,
    MBC5WithRumble = 0x1C,
    MBC5WithRumbleAndBattery = 0x1D,
    MBC5WithRumbleAndBatteryAndRAM = 0x1E,

    PocketCamera = 0xFC,
    BandaiTama5 = 0xFD,
    HuC3 = 0xFE,
    HuC1WithRAMAndBattery = 0xFF,
}

impl CartridgeType {
    fn from_byte(byte: u8) -> crate::Result<Self> {
        match byte {
            0x00 => Ok(Self::RomOnly),

            0x01 => Ok(Self::MBC1),
            0x02 => Ok(Self::MBC1WithRAM),
            0x03 => Ok(Self::MBC1WithRAMAndBattery),

            0x05 => Ok(Self::MBC2),
            0x06 => Ok(Self::MBC2WithBattery),

            0x08 => Ok(Self::ROMAndRAM),
            0x09 => Ok(Self::ROMAndRAMAndBattery),

            0x0B => Ok(Self::MMM01),
            0x0C => Ok(Self::MMM01WithRAM),
            0x0D => Ok(Self::MMM01WithRAMAndBattery),

            0x0F => Ok(Self::MBC3WithTimerAndBattery),
            0x10 => Ok(Self::MBC3WithTimerAndBatteryAndRAM),
            0x11 => Ok(Self::MBC3),
            0x12 => Ok(Self::MBC3WithRAM),
            0x13 => Ok(Self::MBC3WithRAMAndBattery),

            0x15 => Ok(Self::MBC4),
            0x16 => Ok(Self::MBC4WithRAM),
            0x17 => Ok(Self::MBC4WithRAMAndBattery),

            0x19 => Ok(Self::MBC5),
            0x1A => Ok(Self::MBC5WithRAM),
            0x1B => Ok(Self::MBC5WithRAMAndBattery),
            0x1C => Ok(Self::MBC5WithRumble),
            0x1D => Ok(Self::MBC5WithRumbleAndBattery),
            0x1E => Ok(Self::MBC5WithRumbleAndBatteryAndRAM),

            0xFC => Ok(Self::PocketCamera),
            0xFD => Ok(Self::BandaiTama5),
            0xFE => Ok(Self::HuC3),
            0xFF => Ok(Self::HuC1WithRAMAndBattery),

            _ => Err(CartridgeLoadError::InvalidMapper.into()),
        }
    }
}

// TODO: Re-examine the API of this struct.
// ? Do all these fields need to be exposed?
// ? Should this use a builder instead of `from_buffer`/`from_file`?
// ? Should there be flags for colour/SGB compatibility?
/// A Gameboy cartridge.
pub struct Cartridge {
    pub title: String,
    pub cartridge_type: CartridgeType,
    pub bank_count: u8,
    pub ram_size: u8,
    pub is_japanese: bool,
    pub data: Vec<u8>,
}

impl Cartridge {
    pub fn region(&self) -> String {
        if self.is_japanese {
            "Japan".into()
        } else {
            "Worldwide".into()
        }
    }

    pub(crate) fn load_banks(&self, mmu: &mut MMU) {
        mmu.bank0_mut().copy_from_slice(&self.data[0x0000..=0x3FFF]);
        mmu.bank1_mut().copy_from_slice(&self.data[0x4000..=0x7FFF])
    }
}

impl std::fmt::Debug for Cartridge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(Cartridge))
            .field("title", &self.title)
            .field("cartridge_type", &self.cartridge_type)
            .field("bank_count", &self.bank_count)
            .field("ram_size", &self.ram_size)
            .field("is_japanese", &self.is_japanese)
            .field("self.data", &format!("Vec<u8> {}B", self.data.len()))
            .finish()
    }
}

impl<'cart> IntoIterator for &'cart Cartridge {
    type Item = AssemblyInstruction;
    type IntoIter = AssemblyInstructionStream<'cart>;

    fn into_iter(self) -> Self::IntoIter {
        AssemblyInstructionStream::new(self)
    }
}

#[cfg(test)]
impl Default for Cartridge {
    fn default() -> Self {
        Self {
            title: String::from("TEST         "),
            bank_count: 0,
            cartridge_type: CartridgeType::RomOnly,
            is_japanese: false,
            ram_size: 0,
            data: vec![],
        }
    }
}

/// An enum indicating where this cartridge should be loaded from.
enum CartridgeSource<'a> {
    /// No source is specified, meaning a cartridge can't be built.
    Empty,
    /// Load the cartridge from a file.
    File(File),
    /// Load the cartridge from a buffer.
    Buffer(&'a [u8]),
}

pub struct CartridgeBuilder<'a> {
    config: Config,
    source: CartridgeSource<'a>,
}

impl<'a> CartridgeBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_config(mut self, config: &Config) -> Self {
        self.config = *config;
        self
    }

    pub fn with_file(mut self, file: File) -> Self {
        self.source = CartridgeSource::File(file);
        self
    }

    pub fn with_buffer(mut self, buffer: &'a [u8]) -> Self {
        self.source = CartridgeSource::Buffer(buffer);
        self
    }

    pub fn build(self) -> crate::Result<Cartridge> {
        let buffer: Vec<u8> = match self.source {
            CartridgeSource::Empty => {
                return Err(CartridgeLoadError::NoSourceSet.into());
            }
            CartridgeSource::Buffer(buf) => buf.into(),
            CartridgeSource::File(file) => {
                let mut buf_reader = BufReader::new(file);
                let mut buffer = Vec::<u8>::new();

                buf_reader
                    .read_to_end(&mut buffer)
                    .map_err(|e| CartridgeLoadError::FileSystemError(e))?;

                buffer
            }
        };

        if self.config.enable_boot_check {
            Self::validate_cartridge_header(&buffer)?;
        }

        let title = Self::parse_cartridge_title(&buffer)?;
        let bank_count = Self::parse_bank_count(&buffer)?;
        let ram_size = Self::parse_ram_size(&buffer)?;
        let is_japanese = Self::is_japanese(&buffer)?;
        let cartridge_type = Self::parse_cartridge_type(&buffer)?;

        Ok(Cartridge {
            title,
            bank_count,
            ram_size,
            is_japanese,
            cartridge_type,
            data: buffer,
        })
    }

    fn validate_cartridge_header(buffer: &[u8]) -> crate::Result<()> {
        if buffer.len() < 0x134 {
            return Err(CartridgeLoadError::ChecksumFail.into());
        }

        for (index, byte) in buffer[0x0104..=0x0133].iter().enumerate() {
            if *byte != CARTRIDGE_HEADER[index] {
                return Err(CartridgeLoadError::ChecksumFail.into());
            }
        }

        Ok(())
    }

    fn parse_cartridge_title(buffer: &[u8]) -> crate::Result<String> {
        String::from_utf8(buffer[0x134..=0x143].into())
            .map(|s| s.trim_end_matches('\u{0}').to_string())
            .map_err(|e| CartridgeLoadError::InvalidTitle(e).into())
    }

    fn parse_bank_count(buffer: &[u8]) -> crate::Result<u8> {
        let value = match buffer[0x148] {
            0 => 0,
            v @ 1..=7 => 2u8.pow((v + 1).into()),
            52 => 72,
            53 => 80,
            54 => 96,
            c => return Err(CartridgeLoadError::InvalidBankCount(c).into()),
        };

        Ok(value)
    }

    fn parse_ram_size(buffer: &[u8]) -> crate::Result<u8> {
        let value = match buffer[0x149] {
            0 => 0,
            1 => 2,
            2 => 8,
            3 => 32,
            c => return Err(CartridgeLoadError::InvalidRamSize(c).into()),
        };

        Ok(value)
    }

    fn is_japanese(buffer: &[u8]) -> crate::Result<bool> {
        Ok(buffer[0x14A] == 0)
    }

    fn parse_cartridge_type(buffer: &[u8]) -> crate::Result<CartridgeType> {
        CartridgeType::from_byte(buffer[0x147])
    }
}

impl Default for CartridgeBuilder<'_> {
    fn default() -> Self {
        Self {
            config: Config::default(),
            source: CartridgeSource::Empty,
        }
    }
}

#[cfg(test)]
mod tests {
    mod cartridge_builder {
        #[test]
        #[should_panic]
        fn it_validates_the_cartridge_header() {
            todo!("This was part of the cartridge before, and needs to now cover the builder");
        }

        #[test]
        #[should_panic]
        fn it_parses_the_cartridge_title() {
            todo!("This was part of the cartridge before, and needs to now cover the builder");
        }
    }
}
