use ferroboy::State;
use std::env;

fn main() {
    println!("ferroboy v{}", env!("CARGO_PKG_VERSION"));

    let mut state: State = Default::default();
    let mut args = pico_args::Arguments::from_env();
    let path: String = args.value_from_str("--rom").unwrap();

    match state.load_cartridge_from_file(&path) {
        Ok(()) => println!("{:?}", state.cartridge),
        Err(message) => println!("Invalid ROM: {}", message),
    };
}
