use crate::state::State;

pub trait Operation: Sync + std::fmt::Debug {
    fn act(&self, state: &mut State) -> Result<(), String>;
}

pub trait ChainableOperation: Sync + std::fmt::Debug {
    type Output: Operation;
    fn act(&self, state: &mut State) -> Result<Self::Output, String>;
}
