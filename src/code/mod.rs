use std::fmt::Display;

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

    fn bits(self) -> Vec<u8> {
        self.0
    }

    fn bit_as_slices(&self) -> &[u8] {
        self.0.as_slice()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn read_operands(&self, def: Definition) -> (Vec<u16>, u16) {
        let mut operands: Vec<u16> = Vec::new();
        let mut offset: usize = 1;

        for width in def.1.iter() {
            match width {
                2 => {
                    let end = offset + *width as usize;
                    let inst_slice = &self.bit_as_slices()[offset..end];
                    operands.push(u16::from_be_bytes(
                        inst_slice.try_into().expect("Error reading operand"),
                    ));
                    offset += *width as usize;
                }
                _ => (),
            }
        }
        let len = offset - 1;
        (operands, len as u16)
    }

    fn string(inst: &Self) -> String {
        todo!()
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
