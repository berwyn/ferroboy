use std::io::Write;
use std::path::Path;

use ferroboy::{Cartridge, CartridgeBuilder};

#[repr(i32)]
enum ErrorCode {
    BadRom = 1,
    InvalidArguments = 2,
    DisassemblyError = 3,
}

fn main() {
    let mut args = pico_args::Arguments::from_env();

    let path: String = args.value_from_str(["-r", "--rom"]).unwrap();
    let output_path: String = args.value_from_str(["-o", "--output"]).unwrap();
    let quiet: bool = args
        .opt_value_from_str(["-q", "--quiet"])
        .unwrap()
        .unwrap_or(false);

    if let Err(e) = args.finish() {
        bail(ErrorCode::InvalidArguments, e.to_string());
    }

    if !quiet {
        println!("ferroboy-dasm v{}", env!("CARGO_PKG_VERSION"));
    }

    let rom_file = match std::fs::File::open(path) {
        Err(e) => bail(ErrorCode::BadRom, e.to_string()),
        Ok(f) => f,
    };

    let builder = CartridgeBuilder::new().from_file(rom_file);
    let cartridge = match builder.build() {
        Err(e) => bail(ErrorCode::BadRom, e),
        Ok(c) => c,
    };

    match disassemble_rom(&cartridge, &output_path) {
        Ok(()) => {
            if !quiet {
                println!("Disassembly written to {}", output_path)
            }
        }
        Err(e) => {
            bail(
                ErrorCode::DisassemblyError,
                format!("Error during disassembly:\n\t{}", e),
            );
        }
    }
}

fn bail<T: Into<String>>(code: ErrorCode, message: T) -> ! {
    eprintln!("{}", message.into());
    std::process::exit(code as i32);
}

fn disassemble_rom<T: AsRef<Path>>(cartridge: &Cartridge, output_path: &T) -> ferroboy::Result<()> {
    let mut options = std::fs::OpenOptions::new();
    match options.write(true).create(true).open(output_path) {
        Ok(file) => {
            let mut writer = std::io::BufWriter::new(file);

            if let Err(e) = write_header(cartridge, &mut writer) {
                return Err(format!("Unable to write header:\n\t{}", e.to_string()));
            }

            for instruction in cartridge.into_iter() {
                writer
                    .write(format!("{}\n", instruction).as_bytes())
                    .map_err(|e| e.to_string())?;
            }

            Ok(())
        }
        Err(e) => Err(format!("Failed to open file:\n\t{}", e)),
    }
}

fn write_header<T: std::io::Write>(cartridge: &Cartridge, writer: &mut T) -> std::io::Result<()> {
    writer
        .write(format!("; {}\n", cartridge.title).as_bytes())
        .and_then(|_| writer.write(format!("; Type: {:?}\n", cartridge.cartridge_type).as_bytes()))
        .and_then(|_| writer.write(format!("; Banks: {:?}\n", cartridge.bank_count).as_bytes()))
        .and_then(|_| writer.write(format!("; Region: {}\n", cartridge.region()).as_bytes()))
        .map(|_| ())
}
