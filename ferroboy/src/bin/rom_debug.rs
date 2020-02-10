use ferroboy::State;

fn main() {
    let mut state: State = Default::default();
    let mut args = pico_args::Arguments::from_env();
    let path: String = args.value_from_str("--rom").unwrap();

    state.load_cartridge_from_file(&path).unwrap();

    println!("{:?}", state.cartridge);
}
