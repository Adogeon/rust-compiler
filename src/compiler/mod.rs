use super::ast;
use super::code;
use super::object;

struct Bytecode {
    instruction: code::Instructions,
    constants: Vec<object::Object>,
}

struct Compiler {
    instructions: code::Instructions,
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
        return Err(format!("can't compile"));
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
