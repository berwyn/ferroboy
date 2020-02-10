use crate::helpers::word_to_u16;
use crate::operations::Operation;
use crate::state::State;
use crate::system::Register;

pub struct Load8ImmediateOperation(pub Register);

impl Operation for Load8ImmediateOperation {
    fn act(&self, state: &mut State) -> Result<(), String> {
        let value = state.read_byte()?;
        state.cpu.set(self.0, value).map(|_| ())
    }
}

pub struct Load8RegisterCopyOperation(pub Register, pub Register);

impl Operation for Load8RegisterCopyOperation {
    fn act(&self, state: &mut State) -> Result<(), String> {
        let value = state.cpu.get(self.1)?;
        state.cpu.set(self.0, value).map(|_| ())
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

        state.cpu.set(self.0, value).map(|_| ())
    }
}

pub struct Load8RegisterToMemoryOperation(pub Register, pub Register);

impl Operation for Load8RegisterToMemoryOperation {
    fn act(&self, state: &mut State) -> Result<(), String> {
        let address = match self.0 {
            Register::PC | Register::BC | Register::DE | Register::HL => state.cpu.get16(self.0)?,
            Register::C => u16::from(state.cpu.get(self.0)?),
            _ => return Err("Invalid register provided".into()),
        };

        let value = state.cpu.get(self.1)?;
        state.mmu[address] = value;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_loads_an_immediate_into_the_register() {
        let mut state = State::new();
        state.mmu.mutate(|m| m[0x00] = 0xFE);

        let op = Load8ImmediateOperation(Register::B);

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0xFE, state.cpu.get(Register::B).unwrap());
    }

    #[test]
    fn it_loads_a_value_from_one_register_to_another() {
        let mut state = State::new();
        let op = Load8RegisterCopyOperation(Register::B, Register::A);

        state.cpu.set(Register::A, 0xFE).unwrap();

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
        state.cpu.set(Register::H, 0x5E).unwrap();
        state.cpu.set(Register::L, 0x50).unwrap();

        assert_eq!(0x00, state.cpu.get(Register::B).unwrap());

        op.act(&mut state).unwrap();

        assert_eq!(0xFE, state.cpu.get(Register::B).unwrap());
    }

    #[test]
    fn it_writes_a_register_into_memory() {
        let mut state = State::new();
        let op = Load8RegisterToMemoryOperation(Register::PC, Register::A);

        state.cpu.set16(Register::PC, 0x5E50).unwrap();
        state.cpu.set(Register::A, 0xBE).unwrap();

        assert_eq!(0x00, state.mmu[0x5E50]);

        op.act(&mut state).unwrap();

        assert_eq!(0xBE, state.mmu[0x5E50]);
    }
}
