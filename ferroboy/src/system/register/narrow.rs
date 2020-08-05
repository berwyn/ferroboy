/// `Register` is an enum to help indicate which registers
/// an operation should apply to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Register {
    /// The accumulator.
    A,
    /// General purpose register
    B,
    /// General purpose register
    C,
    /// General purpose register
    D,
    /// General purpose register
    E,
    /// General purpose register
    F,
    /// General purpose register
    H,
    /// General purpose register
    L,
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Register::A => "A",
                Register::B => "B",
                Register::C => "C",
                Register::D => "D",
                Register::E => "E",
                Register::F => "F",
                Register::H => "H",
                Register::L => "L",
            }
        )
    }
}
