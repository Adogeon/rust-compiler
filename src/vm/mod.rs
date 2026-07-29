use crate::code::{self, Instruction};
use crate::compiler::{Bytecode, Compiler};
use crate::object::Object;

const STACKSIZE: usize = 2048;
const TRUE: Object = Object::BOOLEAN(true);
const FALSE: Object = Object::BOOLEAN(false);

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
                code::OP_TRUE | code::OP_FALSE => {
                    if op == code::OP_TRUE {
                        self.push(TRUE)?;
                    } else {
                        self.push(FALSE)?;
                    }
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
        let right = self.pop();
        let left = self.pop();

        if matches!(left, Object::INTEGER(_)) && matches!(right, Object::INTEGER(_)) {
            let Object::INTEGER(left_value) = left else {
                return Err(format!("Left is not an Integer Object"));
            };
            let Object::INTEGER(right_value) = right else {
                return Err(format!("Right is not an Integer Object"));
            };

            println!("left: {}", left_value);
            println!("right: {}", right_value);
            println!("op: {}", op);

            return Ok(Object::INTEGER(self.execture_binary_integer_operation(
                op,
                left_value,
                right_value,
            )));
        }

        Err(format!(
            "There is not support for {} and {}",
            left.ob_type(),
            right.ob_type()
        ))
    }

    fn execture_binary_integer_operation(&self, op: code::Opcode, left: i64, right: i64) -> i64 {
        match op {
            code::OP_ADD => left + right,
            code::OP_SUB => left - right,
            code::OP_MUL => left * right,
            code::OP_DIV => left / right,
            _ => 0,
        }
    }

    pub fn last_popped_stack_elm(&self) -> Object {
        self.stack[self.sp].clone()
    }
}

#[cfg(test)]
mod test;
