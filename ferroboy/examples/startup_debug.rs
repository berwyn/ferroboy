use ferroboy::{start, tick, CartridgeBuilder, StateBuilder};

fn main() {
    println!("ferroboy v{}", env!("CARGO_PKG_VERSION"));

    let mut args = pico_args::Arguments::from_env();
    let path: String = args.value_from_str(["-r", "--rom"]).unwrap();

    match std::fs::File::open(&path) {
        Ok(file) => {
            let builder = CartridgeBuilder::new().with_file(file);

            match builder.build() {
                Ok(cart) => {
                    println!("Loaded cartridge");
                    let mut state = StateBuilder::new().with_cartridge(cart).build();

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
                Err(message) => panic!(message),
            }
        }
        Err(_) => panic!("Couldn't open file {}", path),
    }
}
