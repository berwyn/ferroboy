use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::system::Config;
use crate::system::MMU;

const CARTRIDGE_HEADER: [u8; 48] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

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
    fn from_byte(byte: u8) -> Result<Self, String> {
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
            _ => Err("Invalid cartridge type!".into()),
        }
    }
}

#[derive(Debug)]
pub struct Cartridge {
    pub title: String,
    pub cartridge_type: CartridgeType,
    pub bank_count: u8,
    pub ram_size: u8,
    pub is_japanese: bool,
    pub data: Vec<u8>,
}

impl Cartridge {
    pub(crate) fn from_buffer(buffer: &[u8], config: &Config) -> Result<Self, String> {
        let title = if config.enable_boot_check {
            Self::parse_cartridge_header(&buffer)?;
            Self::parse_cartridge_title(buffer)?
        } else {
            "UNKNOWN".to_string()
        };

        let cartridge_type = CartridgeType::from_byte(buffer[0x147])?;
        let bank_count = Self::parse_bank_count(buffer)?;
        let ram_size = Self::parse_ram_size(buffer)?;
        let is_japanese = buffer[0x14A] == 0;
        let data = Vec::from(buffer);

        Ok(Self {
            title,
            cartridge_type,
            bank_count,
            ram_size,
            is_japanese,
            data,
        })
    }

    pub(crate) fn from_file(file: File, config: &Config) -> Result<Self, String> {
        let mut buf_reader = BufReader::new(file);
        let mut buffer = Vec::<u8>::new();

        buf_reader
            .read_to_end(&mut buffer)
            .map_err(|e| e.to_string())?;

        Self::from_buffer(&buffer, config)
    }

    pub(crate) fn load_banks(&self, mmu: &mut MMU) {
        mmu.bank0_mut().copy_from_slice(&self.data[0x0000..=0x3FFF]);
        mmu.bank1_mut().copy_from_slice(&self.data[0x4000..=0x7FFF])
    }

    fn parse_cartridge_header(buffer: &[u8]) -> Result<(), String> {
        if buffer.len() < 0x134 {
            return Err("Invalid cartridge header!".into());
        }

        for (index, byte) in buffer[0x0104..=0x0133].iter().enumerate() {
            if *byte != CARTRIDGE_HEADER[index] {
                return Err("Invalid cartridge header!".into());
            }
        }

        Ok(())
    }

    fn parse_cartridge_title(buffer: &[u8]) -> Result<String, String> {
        String::from_utf8(buffer[0x134..=0x143].into())
            .map(|s| s.trim_end_matches('\u{0}').to_string())
            .map_err(|_| "Invalid cartridge title".to_string())
    }

    fn parse_bank_count(buffer: &[u8]) -> Result<u8, String> {
        let value = match buffer[0x148] {
            0 => 0,
            v @ 1..=7 => 2u8.pow((v + 1).into()),
            52 => 72,
            53 => 80,
            54 => 96,
            _ => return Err("Invalid cartridge ROM bank count!".to_string()),
        };

        Ok(value)
    }

    fn parse_ram_size(buffer: &[u8]) -> Result<u8, String> {
        let value = match buffer[0x149] {
            0 => 0,
            1 => 2,
            2 => 8,
            3 => 32,
            _ => return Err("Invalid cartridge RAM size!".to_string()),
        };

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_validates_the_cartridge_header() {
        let mut buffer = vec![0; 0x200];

        assert!(Cartridge::parse_cartridge_header(&buffer).is_err());

        buffer = vec![0; 0x104]
            .iter()
            .chain(CARTRIDGE_HEADER.iter())
            .cloned()
            .collect();

        assert!(Cartridge::parse_cartridge_header(&buffer).is_ok());
    }

    #[test]
    fn it_parses_the_cartridge_title() {
        let title = [
            0x54, 0x45, 0x54, 0x52, 0x49, 0x53, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];

        let buffer = vec![0; 0x134]
            .iter()
            .chain(title.iter())
            .cloned()
            .collect::<Vec<u8>>();

        assert_eq!("TETRIS", Cartridge::parse_cartridge_title(&buffer).unwrap());
    }
}
