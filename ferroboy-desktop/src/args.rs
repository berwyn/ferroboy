pub struct Args {
    pub rom_path: String,
    pub should_step: bool,
}

impl Args {
    pub fn new() -> Self {
        let mut args = pico_args::Arguments::from_env();

        Self {
            rom_path: args.value_from_str(["-r", "--rom"]).unwrap(),
            should_step: args.contains(["-s", "--step"]),
        }
    }
}
