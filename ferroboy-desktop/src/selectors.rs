use druid::Selector;

pub const SELECTOR_STEP: Selector<druid::ExtEventSink> = Selector::new("ferroboy.step");
pub const SELECTOR_STEP_COMPLETE: Selector = Selector::new("ferroboy.step-complete");
