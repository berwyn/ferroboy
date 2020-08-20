use std::io::Write;
use std::path::Path;

use ferroboy::{Cartridge, CartridgeBuilder};

fn main() -> eyre::Result<()> {
    jane_eyre::install()?;

    let mut args = pico_args::Arguments::from_env();

    let path: String = args.value_from_str(["-r", "--rom"])?;
    let output_path: String = args.value_from_str(["-o", "--output"])?;
    let quiet: bool = args
        .opt_value_from_str(["-q", "--quiet"])
        .unwrap()
        .unwrap_or(false);

    args.finish()?;

    if !quiet {
        println!("ferroboy-dasm v{}", env!("CARGO_PKG_VERSION"));
    }

    let rom_file = std::fs::File::open(path)?;

    let cartridge = CartridgeBuilder::new().with_file(rom_file).build()?;

    disassemble_rom(&cartridge, &output_path)?;

    if !quiet {
        println!("Disassembly written to {}", output_path);
    }

    Ok(())
}

fn disassemble_rom<T: AsRef<Path>>(cartridge: &Cartridge, output_path: &T) -> eyre::Result<()> {
    let mut options = std::fs::OpenOptions::new();
    let file = options.write(true).create(true).open(output_path)?;
    let mut writer = std::io::BufWriter::new(file);

    write_header(cartridge, &mut writer)?;

    for instruction in cartridge.into_iter() {
        writer.write(format!("{}\n", instruction).as_bytes())?;
    }

    Ok(())
}

fn write_header<T: std::io::Write>(cartridge: &Cartridge, writer: &mut T) -> std::io::Result<()> {
    writer
        .write(format!("; {}\n", cartridge.title).as_bytes())
        .and_then(|_| writer.write(format!("; Type: {:?}\n", cartridge.cartridge_type).as_bytes()))
        .and_then(|_| writer.write(format!("; Banks: {:?}\n", cartridge.bank_count).as_bytes()))
        .and_then(|_| writer.write(format!("; Region: {}\n", cartridge.region()).as_bytes()))
        .map(|_| ())
}
