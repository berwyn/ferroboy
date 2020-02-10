#[allow(clippy::all)]
use ferroboy_core::FerroboyCore;
use libretro_backend::libretro_core;

mod ferroboy_core;

libretro_core!(FerroboyCore);
