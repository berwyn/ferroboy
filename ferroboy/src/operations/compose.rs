use crate::operations::Operation;
use crate::State;

struct Compose2<A, B>
where
    A: Operation,
    B: Operation,
{
    a: A,
    b: B,
}

impl<A, B> Operation for Compose2<A, B>
where
    A: Operation,
    B: Operation,
{
    fn act(&self, state: &mut State) -> Result<(), String> {
        self.a.act(state).and_then(|_| self.b.act(state))
    }
}

pub fn compose_operations<A, B>(a: A, b: B) -> impl Operation
where
    A: Operation,
    B: Operation,
{
    Compose2 { a, b }
}
