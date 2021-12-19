use crate::secp256k1::{S256Point, Signature};
use crate::transaction::{Transaction, SigHash, ZProvider};
use crate::util::hash;
use super::{CommandElement, Opcode, Num, Stack};
use super::error::Error;

pub fn verify_tx(tx: &Transaction) -> Result<bool, Error> {
    let z_provider = Box::new(tx.clone()) as Box<dyn ZProvider>;

    let mut amount_in = 0;
    for (i, input) in tx.inputs.iter().enumerate() {
        let output_ref = input.get_output_ref()?;
        let combined_script = output_ref.script().clone() + input.script.clone();
        if !combined_script.evaluate(i, &z_provider)? {
            return Ok(false);
        }
        amount_in += output_ref.amount();
    }

    let mut amount_out = 0;
    for output in &tx.outputs {
        amount_out += output.amount();
    }
    if amount_in < amount_out {
        return Err(Error::InvalidTxFee)
    }

    Ok(true)
}

pub fn check_signature(pk: Vec<u8>, sig_raw: Vec<u8>, index: usize, z_privoder: &Box<dyn ZProvider>) -> Result<bool, Error>  {
    let pk = S256Point::parse(&pk).map_err(|_| Error::InvalidPublicKey)?;
    let (sig, used) = Signature::parse_der(&sig_raw).map_err(|_| Error::InvalidSignature)?;

    let sighash = if used + 1 == sig_raw.len() {
        SigHash::parse(sig_raw[used])?
    } else {
        SigHash::All // default is all
    };
    let z = z_privoder.z_u256(index, sighash)?;

    Ok(sig.verify(z, pk))
}

pub fn evaluate_command(cmd: CommandElement, stack: &mut Stack, index: usize, z_privoder: &Box<dyn ZProvider>) -> Result<bool, Error> {
    let mut result = true;
    match cmd {
        CommandElement::Op(op) => result = evaluate_opcode(op, stack, index, z_privoder)?,
        CommandElement::Data(data) => stack.push(data),
    };

    Ok(result)
}

fn evaluate_opcode(op: Opcode, stack: &mut Stack, index: usize, z_privoder: &Box<dyn ZProvider>) -> Result<bool, Error> {
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
            let ele = hash::hash160(&ele).to_vec();
            stack.push(ele);
        },
        Opcode::OpChecksig => {
            let pk = stack.pop()?;
            let sig = stack.pop()?;
            result = check_signature(pk, sig, index, z_privoder)?;

            let stack_result = if result { vec![1] } else { vec![] };
            stack.push(stack_result);
        },
    };
    Ok(result)
}

#[cfg(test)]
mod tests {
    #[test]
    fn operator_verify_transaction() {
        use crate::transaction::{TxFetcher};

        let mut tx_hash = [0u8; 32];
        tx_hash.copy_from_slice(&hex::decode("f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16").unwrap());
        let mut fetcher = TxFetcher::new();
        let first_tx = fetcher.fetch(&tx_hash, false, false).unwrap();

        let result = super::verify_tx(&first_tx).unwrap();
        assert!(result);
    }
}
