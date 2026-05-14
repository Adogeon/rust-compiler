use std::fmt::{Display, Write};

type Opcode = u8;

pub const OP_CONSTANT: Opcode = 0b0;

type Definition = (&'static str, Vec<u16>);

fn look_up(op_code: Opcode) -> Option<Definition> {
    match op_code {
        OP_CONSTANT => Some(("OpConstant", vec![2])),
        _ => None,
    }
}

#[derive(Debug)]
struct Instruction(Vec<u8>);

impl Instruction {
    fn make(op_code: Opcode, operands: &[u16]) -> Self {
        match op_code {
            OP_CONSTANT => {
                let mut result = vec![op_code];
                let encode_bytes = &operands[0].to_be_bytes();
                result.extend_from_slice(encode_bytes);
                Self(result)
            }
            _ => Self(Vec::new()),
        }
    }

    fn bits(&self) -> &Vec<u8> {
        &self.0
    }

    fn bit_as_slices(&self) -> &[u8] {
        self.0.as_slice()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn read_operands(def: &Definition, instruction: &[u8]) -> (Vec<u16>, u16) {
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

    fn string(inst: &Self) -> String {
        let mut buffer = String::new();
        let mut step = 0;
        let inst_slice = inst.bit_as_slices();
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
            1 => format!("{} {}", def.0, operands[0]),
            _ => format!("Error: unhandle operand_count for {}", def.0),
        }
    }
}

pub fn make(op_code: Opcode, operands: &[u16]) -> Vec<u8> {
    match op_code {
        OP_CONSTANT => {
            let mut result = vec![op_code];
            let encode_bytes = &operands[0].to_be_bytes();
            result.extend_from_slice(encode_bytes);
            result
        }
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod test;
