use ferroboy::CartridgeBuilder;
use std::env;

fn main() {
    println!("ferroboy v{}", env!("CARGO_PKG_VERSION"));
    let mut args = pico_args::Arguments::from_env();
    let path: String = args.value_from_str(["-r", "--rom"]).unwrap();

    match std::fs::File::open(&path) {
        Ok(file) => {
            let builder = CartridgeBuilder::new().with_file(file);

            match builder.build() {
                Ok(cart) => println!("{:?}", cart),
                Err(message) => println!("Invalid ROM: {}", message),
            }
        }
        Err(_) => panic!("Couldn't open file {}", path),
    }
}
