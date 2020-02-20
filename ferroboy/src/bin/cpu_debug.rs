use ferroboy::start;
use ferroboy::ConfigBuilder;
use ferroboy::StateBuilder;

pub fn main() {
    println!("ferroboy v{}", env!("CARGO_PKG_VERSION"));

    let buf = include_bytes!("../../assets/gb-test-roms/cpu_instrs/cpu_instrs.gb");
    let config = ConfigBuilder::new().without_boot_check().build();
    let mut state = StateBuilder::new().with_config(config).build();

    if let Ok(()) = state.load_cartridge_from_buffer(buf) {
        start(&mut state).unwrap();
    }
}
