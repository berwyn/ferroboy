use core::convert::TryInto;

use crate::state::State;

pub struct AssemblyInstruction {
    command: String,
    args: [Option<String>; 2],
}

impl std::fmt::Display for AssemblyInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.command)?;

        if let Some(arg) = &self.args[0] {
            write!(f, " {}", arg)?;
        }

        if let Some(arg) = &self.args[1] {
            write!(f, " {}", arg)?;
        }

        Ok(())
    }
}

pub(crate) struct AssemblyInstructionBuilder {
    command: Option<String>,
    args: [Option<String>; 2],
}

impl AssemblyInstructionBuilder {
    pub fn new() -> Self {
        Self {
            command: None,
            args: [None, None],
        }
    }

    pub fn with_command(mut self, command: impl ToString) -> Self {
        self.command = Some(command.to_string());
        self
    }

    pub fn with_arg(mut self, arg: impl ToString) -> Self {
        if self.args[0].is_none() {
            self.args[0].replace(arg.to_string());
        } else if self.args[1].is_none() {
            self.args[1].replace(arg.to_string());
        }

        self
    }

    pub fn build(self) -> crate::Result<AssemblyInstruction> {
        if self.command.is_none() {
            return Err("Command is not set!".to_string());
        }

        Ok(AssemblyInstruction {
            command: self.command.unwrap(),
            args: self.args,
        })
    }
}

pub trait Disassemble {
    fn disassemble(self, state: &State) -> crate::Result<AssemblyInstruction>;
}

impl<T> Disassemble for T
where
    T: TryInto<AssemblyInstruction>,
    T::Error: ToString,
{
    fn disassemble(self, _: &State) -> crate::Result<AssemblyInstruction> {
        self.try_into().map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assembly_instruction_builder_allows_no_args() {
        let res = AssemblyInstructionBuilder::new()
            .with_command("jmp")
            .build();

        assert!(res.is_ok());
        assert_eq!(res.unwrap().command, String::from("jmp"));
    }

    #[test]
    fn assembly_instruction_builder_allows_one_arg() {
        let res = AssemblyInstructionBuilder::new()
            .with_command("jmp")
            .with_arg("$100")
            .build();

        assert!(res.is_ok());

        let instruction = res.unwrap();

        assert_eq!(instruction.command, String::from("jmp"));
        assert_eq!(instruction.args, [Some(String::from("$100")), None]);
    }

    #[test]
    fn assembly_instruction_builder_allows_two_args() {
        let res = AssemblyInstructionBuilder::new()
            .with_command("ld")
            .with_arg("(A)")
            .with_arg("$100")
            .build();

        assert!(res.is_ok());

        let instruction = res.unwrap();

        assert_eq!(instruction.command, String::from("ld"));
        assert_eq!(instruction.args[0], Some(String::from("(A)")));
        assert_eq!(instruction.args[1], Some(String::from("$100")));
    }

    #[test]
    fn assembly_instruction_builder_ignores_excess_args() {
        let res = AssemblyInstructionBuilder::new()
            .with_command("ld")
            .with_arg("A")
            .with_arg("B")
            .with_arg("C")
            .build();

        assert!(res.is_ok());

        let instruction = res.unwrap();

        assert_eq!(instruction.command, String::from("ld"));
        assert_eq!(instruction.args[0], Some(String::from("A")));
        assert_eq!(instruction.args[1], Some(String::from("B")));
    }

    use crate::system::Register;
    struct TestOperation(pub Register, pub Register);

    impl core::convert::TryFrom<TestOperation> for AssemblyInstruction {
        type Error = String;

        fn try_from(
            value: TestOperation,
        ) -> core::result::Result<AssemblyInstruction, Self::Error> {
            AssemblyInstructionBuilder::new()
                .with_command("TEST")
                .with_arg(value.0)
                .with_arg(value.1)
                .build()
        }
    }

    #[test]
    fn disassemble_is_implemented_for_try_from() {
        use crate::State;

        let res = TestOperation(Register::A, Register::D).disassemble(&State::default());

        assert!(res.is_ok());
        assert_eq!("TEST A D", res.unwrap().to_string());
    }
}