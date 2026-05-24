use super::ast;
use super::code::Instruction;
use super::object;

struct Bytecode {
    instruction: Instruction,
    constants: Vec<object::Object>,
}

struct Compiler {
    instructions: Vec<Instruction>,
    constants: Vec<object::Object>,
}

impl Compiler {
    fn new() -> Self {
        Compiler {
            instructions: Vec::new(),
            constants: Vec::new(),
        }
    }

    fn compile(&self, node: impl ast::IsNode) -> Result<(), String> {
        Ok(())
    }

    fn bytecode(&self) -> Bytecode {
        Bytecode {
            instruction: Instruction::from_bits(Vec::new()),
            constants: self.constants.clone(),
        }
    }
}

#[cfg(test)]
mod test;
