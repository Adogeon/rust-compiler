use crate::code::{self, Instruction};
use crate::compiler::{Bytecode, Compiler};
use crate::object::Object;

const STACKSIZE: usize = 2048;

pub struct VM {
    constants: Vec<Object>,
    instruction: Instruction,
    stack: Vec<Object>,
    sp: usize,
}

impl VM {
    pub fn new(bytecode: Bytecode) -> Self {
        VM {
            instruction: bytecode.instruction,
            constants: bytecode.constants,
            stack: Vec::with_capacity(STACKSIZE),
            sp: 0,
        }
    }
    pub fn run(&mut self) -> Result<(), String> {
        let inst_slices = self.instruction.bits().clone();
        let mut ip = 0;
        while ip < inst_slices.len() {
            let op = inst_slices[ip];
            match op {
                code::OP_CONSTANT => {
                    let offset = ip + 1;
                    let end = offset + 2;
                    let const_index = u16::from_be_bytes(
                        inst_slices[offset..end]
                            .try_into()
                            .expect("Error reading operand"),
                    );
                    let cst = self.constants[const_index as usize].clone();
                    self.push(cst)?;
                    ip += 2;
                }
                code::OP_ADD | code::OP_MUL | code::OP_SUB | code::OP_DIV => {
                    let result = self.execute_binary_operation(op)?;
                    self.push(result)?
                }
                code::OP_POP => {
                    self.pop();
                }
                _ => todo!(),
            }
            ip += 1;
        }

        Ok(())
    }
    pub fn stack_top(&self) -> Option<Object> {
        if self.sp == 0 {
            None
        } else {
            Some(self.stack[self.sp - 1].clone())
        }
    }

    fn push(&mut self, o: Object) -> Result<(), String> {
        if self.sp >= STACKSIZE {
            return Err(String::from("Stack overflow"));
        }

        if self.sp == 0 && self.stack.len() == 0 {
            self.stack.push(o)
        } else {
            self.stack.insert(self.sp, o);
        }

        self.sp += 1;

        Ok(())
    }

    fn pop(&mut self) -> Object {
        let result = self.stack[self.sp - 1].clone();
        self.sp -= 1;
        result
    }

    fn execute_binary_operation(&mut self, op: code::Opcode) -> Result<Object, String> {
        let left_value = if let Object::INTEGER(val) = self.pop() {
            val
        } else {
            return Err(format!("object is not interger"));
        };
        let right_value = if let Object::INTEGER(val) = self.pop() {
            val
        } else {
            return Err(format!("object is not interger"));
        };
        let result = match op {
            code::OP_ADD => left_value + right_value,
            code::OP_SUB => left_value - right_value,
            code::OP_MUL => left_value * right_value,
            code::OP_DIV => left_value / right_value,
            _ => return Err(format!("Something is terribly wrong!")),
        };
        Ok(Object::INTEGER(result))
    }

    pub fn last_popped_stack_elm(&self) -> Object {
        self.stack[self.sp].clone()
    }
}

#[cfg(test)]
mod test;
