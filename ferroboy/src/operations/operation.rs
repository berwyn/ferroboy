use crate::state::State;

/// An action that can be taken on the system.
///
/// Operations should, ideally, encapsulate all the changes
/// to the system that occur as the result of an instruction.
/// This includes things like mutating registers, changing
/// flags, writing to memory, etc.
pub trait Operation: Sync + std::fmt::Debug {
    fn act(&self, state: &mut State) -> crate::Result<()>;
}
