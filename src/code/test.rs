use super::*;

#[test]
fn test_make() -> Result<(), String> {
    let test_cases: Vec<(u8, &[u16], Vec<u8>)> =
        vec![(OP_CONSTANT, &[65534], vec![OP_CONSTANT, 255, 254])];

    for (op, operands, expected) in test_cases {
        let instruction: Instruction = Instruction::make(op, operands);

        if instruction.len() != expected.len() {
            return Err(format!(
                "instruction has wrong length. want {}, got {}",
                expected.len(),
                instruction.len()
            ));
        }

        for (i, b) in expected.iter().enumerate() {
            if instruction.slices()[i] != expected[i] {
                return Err(format!(
                    "wrong byte at position {}, want {}, got {}",
                    i,
                    b,
                    instruction.slices()[i]
                ));
            }
        }
    }

    Ok(())
}

#[test]
fn test_instructions_string() -> Result<(), String> {
    let instructions = vec![
        make(OP_CONSTANT, &[1]),
        make(OP_CONSTANT, &[2]),
        make(OP_CONSTANT, &[65535]),
    ];

    let expected = "000 OpConstant 1\n003 OpConstant 2\n006 OpConstant 65535\n";

    let concatted = Instruction(instructions.concat());
    assert_eq!(
        Instruction::string(&concatted),
        expected,
        "instructions wrongly formatted. want = {}, got = {}",
        expected,
        Instruction::string(&concatted)
    );

    Ok(())
}

#[test]
fn test_read_operands() -> Result<(), String> {
    let test_cases: Vec<(Opcode, &[u16], u16)> = vec![(OP_CONSTANT, &[65535], 2)];

    for tc in test_cases {
        let instruction = Instruction::make(tc.0, tc.1);

        if let Some(def) = look_up(tc.0) {
            let (operand_read, n) = Instruction::read_operands(&def, &instruction.0[1..]);
            assert_eq!(n, tc.2, "n wrong. want={}, got={}", tc.2, n);
            for (i, want) in tc.1.iter().enumerate() {
                assert_eq!(
                    operand_read[i], *want,
                    "operand wrong. want={}, got={}",
                    want, operand_read[i]
                )
            }
        } else {
            return Err(format!("definition not found"));
        }
    }
    Ok(())
}
