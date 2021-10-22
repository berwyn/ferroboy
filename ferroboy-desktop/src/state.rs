use std::sync::{Arc, RwLock};

pub type State = Arc<RwLock<ferroboy::State>>;
