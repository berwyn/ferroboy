use std::io::Write;
use std::path::Path;

use ferroboy::State;

fn main() {
    println!("ferroboy-dasm v{}", env!("CARGO_PKG_VERSION"));

    let mut state = State::default();
    let mut args = pico_args::Arguments::from_env();
    let path: String = args.value_from_str("--rom").unwrap();
    let output_path: String = args.value_from_str("-o").unwrap();

    match state.load_cartridge_from_file(&path) {
        Ok(()) => println!("{:?}", state.cartridge),
        Err(message) => println!("Invalid ROM: {}", message),
    };

    if let Err(e) = ferroboy::start(&mut state) {
        println!("Cartridge failed startup: {}", e);
    }

    match disassemble_rom(&mut state, &output_path) {
        Ok(()) => println!("Disassembly written to {}", output_path),
        Err(e) => println!("Error during disassembly: \n\t{:?}", e),
    }
}

fn disassemble_rom<T: AsRef<Path>>(state: &mut State, output_path: &T) -> ferroboy::Result<()> {
    if let Ok(file) = std::fs::File::open(output_path) {
        let mut writer = std::io::BufWriter::new(file);

        ferroboy::start(state)?;

        while !state.is_halted() {
            match ferroboy::tick(state) {
                Ok(operation) => {
                    let result = writer
                        .write(format!("Operation: {:?}", operation).as_bytes())
                        .and_then(|_| writer.write(format!("State: {:?}", state).as_bytes()));

                    if let Err(e) = result {
                        return Err(e.to_string());
                    }
                }
                Err(message) => return Err(message),
            }
        }
    }

    Err("Unable to open output file".into())
}
