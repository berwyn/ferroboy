use crate::state::State;

pub trait Operation: Sync {
    fn act(&self, state: &mut State) -> Result<(), String>;
}
