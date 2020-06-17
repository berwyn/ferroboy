use ferroboy::start;
use ferroboy::tick;
use ferroboy::State;

fn main() {
    println!("ferroboy v{}", env!("CARGO_PKG_VERSION"));

    let mut state: State = Default::default();
    let mut args = pico_args::Arguments::from_env();
    let path: String = args.value_from_str("--rom").unwrap();

    match state.load_cartridge_from_file(&path) {
        Ok(()) => {
            println!("Loaded cartridge");

            start(&mut state).unwrap();
            println!("Startup succeeded");
            println!("\t{:?}", state.cpu);

            for _ in 1..4 {
                match tick(&mut state) {
                    Ok(_) => continue,
                    Err(msg) => println!("\t{}", msg),
                }
            }
        }
        Err(message) => {
            println!("Couldn't start: {}", message);
        }
    }
}
