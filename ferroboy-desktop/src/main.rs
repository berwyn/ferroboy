use std::sync::{Arc, RwLock};

use druid::{AppLauncher, PlatformError, WindowDesc};

mod args;
mod delegate;
mod selectors;
mod state;
mod widgets;

const DMG_CPU_CLOCK_DURATION: f32 = 1. / 4.194304;

fn main() -> Result<(), PlatformError> {
    let args = args::Args::new();

    let state = ferroboy::StateBuilder::new()
        .with_config(ferroboy::Config::default())
        .with_cartridge(
            ferroboy::CartridgeBuilder::new()
                .with_file(std::fs::File::open(args.rom_path).unwrap())
                .build()
                .unwrap(),
        )
        .build();

    let mut state = crate::state::State(state);

    prep_emulation(&mut state);

    // If we're in step mode, we don't need to spawn the background thread to
    // step the emulation, since we'll only step when the user asks us to.
    if !args.should_step {
        run_emulation(&state);
    }

    let main_window = WindowDesc::new(widgets::ui_builder).title("Ferroboy");

    AppLauncher::with_window(main_window)
        .delegate(delegate::TopLevelDelegate)
        .use_simple_logger()
        .launch(state)
}

fn prep_emulation(state: &mut state::State) {
    // TODO: This doesn't actually deal with boot ROM
    ferroboy::start(&mut state.0).expect("Couldn't start emulation!");
}

// TODO: This was fine for an Arc but doesn't work like this
fn run_emulation(state: &state::State) {
    let mut target = state.clone();

    std::thread::spawn(move || loop {
        let start = std::time::Instant::now();

        if target.0.is_halted() {
            break;
        } else {
            ferroboy::tick(&mut target.0).expect("Unable to step emulation!");
        }

        let end = std::time::Instant::now();
        let duration = end - start;
        let duration = duration.as_secs_f32();

        std::thread::sleep(std::time::Duration::from_secs_f32(
            DMG_CPU_CLOCK_DURATION - duration,
        ));
    });
}
