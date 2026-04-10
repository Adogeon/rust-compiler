use super::*;

#[test]
fn test_make() -> Result<(), String> {
    let test_cases: Vec<(u8, &[u16], Vec<u8>)> =
        vec![(OP_CONSTANT, &[65534], vec![OP_CONSTANT, 255, 254])];

    for (op, operands, expected) in test_cases {
        let instruction: Vec<u8> = make(op, operands);

        if instruction.len() != expected.len() {
            return Err(format!(
                "instruction has wrong length. want {}, got {}",
                expected.len(),
                instruction.len()
            ));
        }

        for (i, b) in expected.iter().enumerate() {
            if instruction[i] != expected[i] {
                return Err(format!(
                    "wrong byte at position {}, want {}, got {}",
                    i, b, instruction[i]
                ));
            }
        }
    }

    Ok(())
}
