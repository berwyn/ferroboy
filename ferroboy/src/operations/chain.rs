use crate::operations::operation::ChainableOperation;
use crate::operations::operation::Operation;
use crate::state::State;

#[derive(Debug)]
struct Chain2<A>
where
    A: ChainableOperation,
{
    a: A,
}

impl<A> Operation for Chain2<A>
where
    A: ChainableOperation,
{
    fn act(&self, state: &mut State) -> Result<(), String> {
        self.a.act(state).and_then(|result| result.act(state))
    }
}

pub fn chain<T: ChainableOperation>(root: T) -> impl Operation {
    Chain2 { a: root }
}
