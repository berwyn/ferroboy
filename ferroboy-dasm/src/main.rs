use std::io::Write;
use std::path::Path;

use ferroboy::State;

#[repr(i32)]
enum ErrorCode {
    BadRom = 1,
    StartupFailure = 2,
    DisassemblyError = 3,
}

fn main() {
    println!("ferroboy-dasm v{}", env!("CARGO_PKG_VERSION"));

    let mut state = State::default();
    let mut args = pico_args::Arguments::from_env();
    let path: String = args.value_from_str("--rom").unwrap();
    let output_path: String = args.value_from_str("-o").unwrap();

    if let Err(message) = state.load_cartridge_from_file(&path) {
        bail(ErrorCode::BadRom, format!("Invalid ROM: {}", message));
    };

    if let Err(e) = ferroboy::start(&mut state) {
        bail(
            ErrorCode::StartupFailure,
            format!("Cartridge failed startup: \n\t{}", e),
        );
    }

    match disassemble_rom(&mut state, &output_path) {
        Ok(()) => println!("Disassembly written to {}", output_path),
        Err(e) => {
            bail(
                ErrorCode::DisassemblyError,
                format!("Error during disassembly:\n\t{}", e),
            );
        }
    }
}

fn bail<T: Into<String>>(code: ErrorCode, message: T) {
    eprintln!("{}", message.into());
    std::process::exit(code as i32);
}

fn disassemble_rom<T: AsRef<Path>>(state: &mut State, output_path: &T) -> ferroboy::Result<()> {
    let path: &Path = output_path.as_ref();
    let mut options = std::fs::OpenOptions::new();
    match options.write(true).create(true).open(path) {
        Ok(file) => {
            let mut writer = std::io::BufWriter::new(file);

            if let Err(e) = write_header(state, &mut writer) {
                return Err(format!("Unable to write header:\n\t{}", e.to_string()));
            }

            ferroboy::start(state)?;

            while !state.is_halted() {
                match ferroboy::tick(state) {
                    Ok(operation) => {
                        writer
                            .write(format!("{}\n", operation.disassemble(state)?).as_bytes())
                            .map_err(|e| e.to_string())?;
                    }
                    Err(message) => return Err(message),
                }
            }

            Ok(())
        }
        Err(e) => Err(format!("Failed to open file:\n\t{}", e)),
    }
}

fn write_header<T: std::io::Write>(state: &State, writer: &mut T) -> std::io::Result<()> {
    let cartridge = state.cartridge.as_ref().unwrap();

    writer
        .write(format!("; {}\n", cartridge.title).as_bytes())
        .and_then(|_| writer.write(format!("; Type: {:?}\n", cartridge.cartridge_type).as_bytes()))
        .and_then(|_| writer.write(format!("; Banks: {:?}\n", cartridge.bank_count).as_bytes()))
        .and_then(|_| writer.write(format!("; Region: {}\n", cartridge.region()).as_bytes()))
        .map(|_| ())
}
