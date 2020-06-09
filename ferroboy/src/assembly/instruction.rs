/// A raw 6502 assembly instruction.
///
/// This is mostly available for introspection and disassembly.
pub struct AssemblyInstruction {
    command: String,
    args: [Option<String>; 2],
    comment: Option<String>,
}

impl std::fmt::Display for AssemblyInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.command)?;

        if let Some(arg) = &self.args[0] {
            write!(f, " {}", arg)?;
        }

        if let Some(arg) = &self.args[1] {
            write!(f, ",{}", arg)?;
        }

        if let Some(comment) = &self.comment {
            write!(f, " ; {}", comment)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod assembly_instruction_display_tests {
    use super::*;

    #[test]
    fn it_formats_noarg_instructions() {
        let instruction = AssemblyInstruction {
            command: String::from("TEST"),
            args: [None, None],
            comment: None,
        };

        assert_eq!("TEST", instruction.to_string());
    }

    #[test]
    fn it_formats_single_argument_instructions() {
        let instruction = AssemblyInstruction {
            command: String::from("TEST"),
            args: [Some(String::from("A")), None],
            comment: None,
        };

        assert_eq!("TEST A", instruction.to_string());
    }

    #[test]
    fn it_formats_dual_argument_instructions() {
        let instruction = AssemblyInstruction {
            command: String::from("TEST"),
            args: [Some(String::from("A")), Some(String::from("B"))],
            comment: None,
        };

        assert_eq!("TEST A,B", instruction.to_string());
    }

    #[test]
    fn it_formats_comments_correctly() {
        let command = String::from("TEST");
        let lhs = Some(String::from("LEFT"));
        let rhs = Some(String::from("RIGHT"));
        let comment = Some(String::from("Distilled wisdom"));

        let noop = AssemblyInstruction {
            command: command.clone(),
            args: [None, None],
            comment: comment.clone(),
        };

        assert_eq!("TEST ; Distilled wisdom", noop.to_string());

        let single_arg = AssemblyInstruction {
            command: command.clone(),
            args: [lhs.clone(), None],
            comment: comment.clone(),
        };

        assert_eq!("TEST LEFT ; Distilled wisdom", single_arg.to_string());

        let dual_arg = AssemblyInstruction {
            command,
            args: [lhs, rhs],
            comment,
        };

        assert_eq!("TEST LEFT,RIGHT ; Distilled wisdom", dual_arg.to_string());
    }
}

pub(crate) struct AssemblyInstructionBuilder {
    command: Option<String>,
    args: [Option<String>; 2],
    comment: Option<String>,
}

impl AssemblyInstructionBuilder {
    pub fn new() -> Self {
        Self {
            command: None,
            args: [None, None],
            comment: None,
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

    #[allow(unused)]
    pub fn with_comment(mut self, arg: impl ToString) -> Self {
        self.comment.replace(arg.to_string());
        self
    }

    pub fn build(self) -> crate::Result<AssemblyInstruction> {
        if self.command.is_none() {
            return Err("Command is not set!".to_string());
        }

        Ok(AssemblyInstruction {
            command: self.command.unwrap(),
            args: self.args,
            comment: self.comment,
        })
    }
}

#[cfg(test)]
mod assembly_instruction_builder_tests {
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
}
