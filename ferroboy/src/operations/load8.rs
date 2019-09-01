use crate::cpu::Register;
use crate::helpers::word_to_u16;
use crate::operations::Operation;
use crate::state::State;

pub struct Load8ImmediateOperation(pub Register, pub u8);

impl Operation for Load8ImmediateOperation {
    fn act(&self, state: &mut State) -> Result<(), String> {
        state.cpu.set(self.0, |_| self.1).map(|_| ())
    }
}

pub struct Load8RegisterCopyOperation(pub Register, pub Register);

impl Operation for Load8RegisterCopyOperation {
    fn act(&self, state: &mut State) -> Result<(), String> {
        let value = state.cpu.get(self.1)?;
        state.cpu.set(self.0, |_| value).map(|_| ())
    }
}

pub struct Load8FromMemoryOperation(pub Register, pub Register);

impl Operation for Load8FromMemoryOperation {
    fn act(&self, state: &mut State) -> Result<(), String> {
        let (high, low) = Register::to_8bit_pair(self.1)?;

        let address_high = state.cpu.get(high)?;
        let address_low = state.cpu.get(low)?;
        let address = word_to_u16((address_high, address_low));
        let value = state.mmu[address];

        state.cpu.set(self.0, |_| value).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_loads_an_immediate_into_the_register() {
        let mut state = State::new();
        let op = Load8ImmediateOperation(Register::B, 0xFE);

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0xFE, state.cpu.get(Register::B).unwrap());
    }

    #[test]
    fn it_loads_a_value_from_one_register_to_another() {
        let mut state = State::new();
        let op = Load8RegisterCopyOperation(Register::B, Register::A);

        state.cpu.set(Register::A, |_| 0xFE).unwrap();

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());
        assert_eq!(0xFE, state.cpu.get(Register::A).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0xFE, state.cpu.get(Register::B).unwrap());
        assert_eq!(0xFE, state.cpu.get(Register::A).unwrap());
    }

    #[test]
    fn it_loads_a_value_from_memory_to_register() {
        let mut state = State::new();
        let op = Load8FromMemoryOperation(Register::B, Register::HL);

        state.mmu.mutate(|mmu| mmu[0x5E50] = 0xFE);
        state.cpu.set(Register::H, |_| 0x5E).unwrap();
        state.cpu.set(Register::L, |_| 0x50).unwrap();

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0xFE, state.cpu.get(Register::B).unwrap());
    }
}
