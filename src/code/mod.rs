pub type Instructions = Vec<u8>;

type Opcode = u8;

pub const OP_CONSTANT: Opcode = 0;

type Definition = (&'static str, Vec<u16>);

fn look_up(op_code: Opcode) -> Option<Definition> {
    match op_code {
        OP_CONSTANT => Some(("OpConstant", vec![2])),
        _ => None,
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
