use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, RwLock,
};

use druid::{
    widget::{Flex, Image, Label},
    AppLauncher, Env, ImageBuf, PlatformError, Widget, WindowDesc,
};
use ferroboy::State;

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

    let main_window = WindowDesc::new(|| ui_builder());

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

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(state)
}

fn ui_builder() -> impl Widget<Arc<RwLock<State>>> {
    let pixelbuf: &[u8] = &[0u8; 160 * 144];

    let imagebuf = ImageBuf::from_raw(pixelbuf, druid::piet::ImageFormat::Grayscale, 160, 144);

    let image = Image::new(imagebuf);

    Flex::row().with_child(image).with_child(register_table())
}

fn register_table() -> impl Widget<Arc<RwLock<State>>> {
    let mut widget = Flex::column();
    widget.add_child(Label::new("Narrow"));

    for child in ferroboy::Register::variants().map(|r| {
        Flex::row()
            .with_child(Label::new(format!("{}", r)))
            .with_child(Label::new(move |data: &Arc<RwLock<State>>, _env: &Env| {
                if let Ok(data) = data.read() {
                    data.cpu.get(r).to_string()
                } else {
                    "Err".into()
                }
            }))
    }) {
        widget.add_child(child);
    }

    widget.add_child(Label::new("Wide"));

    for child in ferroboy::WideRegister::variants().map(|r| {
        Flex::row()
            .with_child(Label::new(format!("{}", r)))
            .with_child(Label::new(move |data: &Arc<RwLock<State>>, _env: &Env| {
                if let Ok(data) = data.read() {
                    data.cpu.get16(r).to_string()
                } else {
                    "Err".into()
                }
            }))
    }) {
        widget.add_child(child);
    }

    widget
}
