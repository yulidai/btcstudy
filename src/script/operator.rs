use crate::secp256k1::{S256Point, Signature};
use crate::util::hash;
use primitive_types::U256;
use super::{CommandElement, Opcode, operator, Num, Stack};
use super::error::Error;

pub fn check_signature(pk: Vec<u8>, sig: Vec<u8>, z: U256) -> Result<bool, Error>  {
    let pk = S256Point::parse(&pk).map_err(|_| Error::InvalidPublicKey)?;
    let (sig, _) = Signature::parse_der(&sig).map_err(|_| Error::InvalidSignature)?;

    Ok(sig.verify(z, pk))
}

pub fn evaluate_command(cmd: CommandElement, stack: &mut Stack, z: U256) -> Result<bool, Error> {
    let mut result = true;
    match cmd {
        CommandElement::Op(op) => result = evaluate_opcode(op, stack, z)?,
        CommandElement::Data(data) => stack.push(data),
    };

    Ok(result)
}

fn evaluate_opcode(op: Opcode, stack: &mut Stack, z: U256) -> Result<bool, Error> {
    let mut result = true;
    match op {
        Opcode::Op0 => {
            let num = Num::from(0);
            stack.push(num.encode());
        },
        Opcode::Op1 |
        Opcode::Op2 |
        Opcode::Op3 |
        Opcode::Op4 |
        Opcode::Op5 |
        Opcode::Op6 |
        Opcode::Op7 |
        Opcode::Op8 |
        Opcode::Op9 |
        Opcode::Op10 |
        Opcode::Op11 |
        Opcode::Op12 |
        Opcode::Op13 |
        Opcode::Op14 |
        Opcode::Op15 |
        Opcode::Op16 => {
            let num = op.value() - 0x50;
            let num = Num::from(num as i64).encode();
            stack.push(num);
        },
        Opcode::OpDup => {
            let ele = stack.pop()?;
            stack.push(ele.clone());
            stack.push(ele);
        },
        Opcode::OpEqual => {
            result = stack.pop()? == stack.pop()?;
            if result { stack.push(vec![1]) } else { stack.push(vec![]) }; // 0 is empty bytes
        },
        Opcode::OpEqualverify => result = stack.pop()? == stack.pop()?,
        Opcode::OpAdd => {
            let left = stack.pop()?;
            let left = Num::decode(left)?;
            let right = stack.pop()?;
            let right = Num::decode(right)?;

            let result = left + right;
            stack.push(result.encode());
        },
        Opcode::OpHash160 => {
            let ele = stack.pop()?;
            let ele = hash::hash160(&ele);
            stack.push(ele);
        },
        Opcode::OpChecksig => {
            let pk = stack.pop()?;
            let sig = stack.pop()?;
            result = operator::check_signature(pk, sig, z)?;

            let stack_result = if result { vec![1] } else { vec![] };
            stack.push(stack_result);
        },
    };
    Ok(result)
}
