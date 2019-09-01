use crate::state::State;

pub trait Operation {
    fn act(&self, state: &mut State) -> Result<(), String>;
}
