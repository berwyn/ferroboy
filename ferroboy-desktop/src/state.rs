use druid::Data;

#[derive(Clone, PartialEq, Eq)]
pub struct State(pub ferroboy::State);

impl Data for State {
    fn same(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
