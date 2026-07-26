use std::error::Error;

use super::object;
use crate::code;
use crate::{
    ast::{ASTNode, Expression, NodeType, Statement},
    object::Object,
};

pub struct Bytecode {
    pub instruction: code::Instruction,
    pub constants: Vec<object::Object>,
}

pub struct Compiler {
    instructions: Vec<code::Instruction>,
    constants: Vec<object::Object>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            instructions: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn compile(&mut self, node: ASTNode) -> Result<(), String> {
        match node.value {
            NodeType::PNode(program) => {
                for stmts in program.statements {
                    let deref = *stmts;
                    self.compile(deref.into())?;
                }
            }
            NodeType::SNode(stmt) => match stmt {
                Statement::LetStmt(let_stmt) => todo!(),
                Statement::RetStmt(ret_stmt) => todo!(),
                Statement::ExpStmt(exp_stmt) => {
                    self.compile(exp_stmt.expression.into())?;
                    self.emit(code::OP_POP, &[]);
                }
                Statement::BlcStmt(blc_stmt) => todo!(),
            },
            NodeType::ENode(exps) => match exps {
                Expression::Identifier(identifier) => todo!(),
                Expression::StringLit(string_literal) => todo!(),
                Expression::IntLit(integer_literal) => {
                    let int_lit = Object::INTEGER(integer_literal.value);
                    let const_pos = self.add_constant(int_lit);
                    self.emit(code::OP_CONSTANT, &[const_pos as u16]);
                }
                Expression::PreExp(prefix_expression) => todo!(),
                Expression::InExp(infix_expression) => {
                    self.compile(infix_expression.left.clone().into())?;
                    self.compile(infix_expression.right.clone().into())?;
                    match infix_expression.operator.as_str() {
                        "+" => self.emit(code::OP_ADD, &[]),
                        "-" => self.emit(code::OP_SUB, &[]),
                        "*" => self.emit(code::OP_MUL, &[]),
                        "/" => self.emit(code::OP_DIV, &[]),
                        _ => return Err(format!("unknown operator {}", infix_expression.operator)),
                    };
                }
                Expression::BoolLit(boolean) => todo!(),
                Expression::IfExp(if_expression) => todo!(),
                Expression::FncLit(function_literal) => todo!(),
                Expression::CallExp(call_expression) => todo!(),
                Expression::ArrayExp(array_expression) => todo!(),
                Expression::IndexExp(index_expression) => todo!(),
                Expression::HashLit(hash_literal) => todo!(),
            },
        };
        Ok(())
    }

    pub fn bytecode(&self) -> Bytecode {
        Bytecode {
            instruction: code::Instruction::concat_inst(self.instructions.clone()),
            constants: self.constants.clone(),
        }
    }

    fn add_constant(&mut self, obj: Object) -> usize {
        self.constants.push(obj);
        self.constants.len() - 1
    }

    fn emit(&mut self, op: code::Opcode, operands: &[u16]) -> usize {
        let ins = code::Instruction::make(op, operands);
        let pos = self.add_instruction(ins);
        pos
    }

    fn add_instruction(&mut self, ins: code::Instruction) -> usize {
        let pos_new_instruction = self.instructions.len();
        self.instructions.push(ins);
        pos_new_instruction
    }
}

#[cfg(test)]
mod test;
