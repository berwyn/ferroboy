#![deny(nonstandard_style, rust_2018_idioms, future_incompatible)]
#![deny(clippy::all)]

use std::sync::Mutex;

use ferroboy::{tick, State};
use lazy_static::lazy_static;

lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(State::new());
}

#[no_mangle]
extern "C" fn retro_init() {
    lazy_static::initialize(&STATE);
}

#[no_mangle]
extern "C" fn retro_run() {
    let mut lock = STATE.lock().unwrap();
    let state = &mut *lock;
    tick(state).unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
