use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, RwLock,
};

use druid::{AppLauncher, PlatformError, WindowDesc};

mod delegate;
mod selectors;
mod state;
mod widgets;

const DMG_CPU_CLOCK_DURATION: f32 = 1. / 4.194304;

fn main() -> Result<(), PlatformError> {
    let mut args = pico_args::Arguments::from_env();
    let path: String = args.value_from_str(["-r", "--rom"]).unwrap();
    let step_mode = args.contains(["-s", "--step"]);
    let should_step = AtomicBool::new(!step_mode);

    let state = ferroboy::StateBuilder::new()
        .with_config(ferroboy::Config::default())
        .with_cartridge(
            ferroboy::CartridgeBuilder::new()
                .with_file(std::fs::File::open(path).unwrap())
                .build()
                .unwrap(),
        )
        .build();

    let main_window = WindowDesc::new(|| widgets::ui_builder()).title("Ferroboy");

    let state = Arc::new(RwLock::new(state));
    let target = state.clone();

    std::thread::spawn(move || loop {
        let start = std::time::Instant::now();

        if !should_step.load(Ordering::Relaxed) {
            continue;
        }

        match target.write() {
            Ok(mut state) => {
                if state.is_halted() {
                    break;
                } else {
                    ferroboy::tick(&mut state)
                }
            }
            Err(_) => break,
        }
        .unwrap();

        let end = std::time::Instant::now();
        let duration = end - start;
        let duration = duration.as_secs_f32();

        std::thread::sleep(std::time::Duration::from_secs_f32(
            DMG_CPU_CLOCK_DURATION - duration,
        ));
    });

    {
        let mut state = state.write().expect("Couldn't write to startup state!");
        ferroboy::start(&mut state).expect("Couldn't start emulation!");
    }

    AppLauncher::with_window(main_window)
        .delegate(delegate::TopLevelDelegate)
        .use_simple_logger()
        .launch(state)
}
