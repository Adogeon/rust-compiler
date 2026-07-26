use std::fmt::{Display, Write};
pub type Opcode = u8;

pub const OP_CONSTANT: Opcode = 0b0;
pub const OP_POP: Opcode = 0b1;
pub const OP_ADD: Opcode = 0b10;
pub const OP_SUB: Opcode = 0b11;
pub const OP_MUL: Opcode = 0b100;
pub const OP_DIV: Opcode = 0b101;

type Definition = (&'static str, Vec<u16>);

fn look_up(op_code: Opcode) -> Option<Definition> {
    match op_code {
        OP_CONSTANT => Some(("OpConstant", vec![2])),
        OP_ADD => Some(("OpAdd", vec![])),
        OP_POP => Some(("OpPop", vec![])),
        OP_SUB => Some(("OpSub", vec![])),
        OP_MUL => Some(("OpMul", vec![])),
        OP_DIV => Some(("OpDiv", vec![])),
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub struct Instruction(Vec<u8>);

impl Instruction {
    pub fn make(op_code: Opcode, operands: &[u16]) -> Self {
        match op_code {
            OP_CONSTANT => {
                let mut result = vec![op_code];
                let encode_bytes = &operands[0].to_be_bytes();
                result.extend_from_slice(encode_bytes);
                Self(result)
            }
            OP_ADD => Self(vec![op_code]),
            OP_POP => Self(vec![op_code]),
            OP_SUB => Self(vec![op_code]),
            OP_MUL => Self(vec![op_code]),
            OP_DIV => Self(vec![op_code]),
            _ => Self(Vec::new()),
        }
    }

    pub fn from_bits(bits: Vec<u8>) -> Self {
        Instruction(bits)
    }

    pub fn concat_inst(ins: Vec<Instruction>) -> Instruction {
        let mut buffer: Vec<u8> = Vec::new();
        for instruction in ins {
            buffer = [buffer, instruction.0].concat();
        }
        Instruction(buffer)
    }

    pub fn bits(&self) -> &Vec<u8> {
        &self.0
    }

    pub fn slices(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn read_operands(def: &Definition, instruction: &[u8]) -> (Vec<u16>, u16) {
        let mut operands: Vec<u16> = Vec::new();
        let mut offset: usize = 0;

        for width in def.1.iter() {
            match width {
                2 => {
                    let end = offset + *width as usize;
                    let inst_slice = &instruction[offset..end];
                    operands.push(u16::from_be_bytes(
                        inst_slice.try_into().expect("Error reading operand"),
                    ));
                    offset += *width as usize;
                }
                _ => (),
            }
        }
        (operands, offset as u16)
    }

    pub fn string(inst: &Self) -> String {
        let mut buffer = String::new();
        let mut step = 0;
        let inst_slice = inst.slices();
        println!("{:?}", inst_slice);
        println!("{}", inst_slice.len());
        while step < inst_slice.len() {
            if let Some(def) = look_up(inst_slice[step]) {
                let (operands, read) = Instruction::read_operands(&def, &inst_slice[step + 1..]);
                writeln!(
                    buffer,
                    "{:03} {}",
                    step,
                    Instruction::fmt_instruction(&def, &operands)
                )
                .unwrap();
                step = step + 1 + read as usize;
                println!("{}", step);
            } else {
                buffer.push_str(&String::from("Error: can't find instruciton definition"));
                continue;
            }
        }
        buffer
    }

    fn fmt_instruction(def: &Definition, operands: &[u16]) -> String {
        let operand_count = def.1.len();
        if operands.len() != operand_count {
            return format!(
                "Error: operand len {} does not match defined {}\n",
                operands.len(),
                operand_count
            );
        };

        match operand_count {
            0 => format!("{}", def.0),
            1 => format!("{} {}", def.0, operands[0]),
            _ => format!("Error: unhandle operand_count for {}", def.0),
        }
    }
}

#[cfg(test)]
mod test;
