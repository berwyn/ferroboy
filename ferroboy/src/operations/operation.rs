use crate::state::State;

pub trait Operation: Sync + std::fmt::Debug {
    fn act(&self, state: &mut State) -> crate::Result<()>;
}
