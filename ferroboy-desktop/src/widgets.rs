use druid::{
    widget::{Button, Flex, Image, Label},
    Command, Env, ImageBuf, Target, Widget, WidgetExt,
};

pub fn ui_builder() -> impl Widget<crate::state::State> {
    let data_column = Flex::column()
        .with_flex_child(step_button(), 1.0)
        .with_default_spacer()
        .with_flex_child(register_table(), 1.0)
        .expand_width();

    Flex::row()
        .with_child(graphics_buffer())
        .with_flex_child(data_column, 1.0)
}

pub fn graphics_buffer() -> impl Widget<crate::state::State> {
    let pixelbuf: &[u8] = &[0u8; 160 * 144];

    let imagebuf = ImageBuf::from_raw(pixelbuf, druid::piet::ImageFormat::Grayscale, 160, 144);

    Image::new(imagebuf)
}

fn step_button() -> impl Widget<crate::state::State> {
    Button::new("Step").on_click(|context, _data, _env| {
        context.submit_command(Command::new(
            crate::selectors::SELECTOR_STEP,
            (),
            Target::Auto,
        ))
    })
}

fn register_table() -> impl Widget<crate::state::State> {
    let mut widget = Flex::column();
    widget.add_child(Label::new("Narrow"));

    for child in ferroboy::Register::variants().map(|r| {
        Flex::row()
            .with_child(Label::new(format!("{}", r)))
            .with_child(Label::new(move |data: &crate::state::State, _env: &Env| {
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
            .with_child(Label::new(move |data: &crate::state::State, _env: &Env| {
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
