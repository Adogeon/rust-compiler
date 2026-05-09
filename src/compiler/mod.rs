use super::ast;
use super::object;

struct Bytecode {
    instruction: Vec<u8>,
    constants: Vec<object::Object>,
}

struct Compiler {
    instructions: Vec<u8>,
    constants: Vec<object::Object>,
}

impl Compiler {
    fn new() -> Self {
        Compiler {
            instructions: Vec::new(),
            constants: Vec::new(),
        }
    }

    fn compile(&self, node: impl ast::Node) -> Result<(), String> {
        Ok(())
    }

    fn bytecode(&self) -> Bytecode {
        Bytecode {
            instruction: self.instructions.clone(),
            constants: self.constants.clone(),
        }
    }
}

#[cfg(test)]
mod test;
