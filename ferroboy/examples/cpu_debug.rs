use ferroboy::{start, tick, CartridgeBuilder, ConfigBuilder, StateBuilder};

pub fn main() {
    println!("ferroboy v{}", env!("CARGO_PKG_VERSION"));

    let buf = include_bytes!("../assets/gb-test-roms/cpu_instrs/cpu_instrs.gb");
    let config = ConfigBuilder::new().without_boot_check().build();
    let cartridge = CartridgeBuilder::new()
        .with_config(&config)
        .with_buffer(buf)
        .build()
        .unwrap();

    let mut state = StateBuilder::new()
        .with_config(config)
        .with_cartridge(cartridge)
        .build();

    start(&mut state).unwrap();

    if cfg!(debug_assertions) {
        println!("{:?}", state);
    }

    loop {
        match tick(&mut state) {
            Ok(_) => {}
            Err(msg) => std::panic::panic_any(msg),
        }
    }
}
