use crate::secp256k1::{S256Point, Signature};
use crate::transaction::{Transaction, SigHash, ZProvider};
use crate::util::{hash, varint};
use super::{CommandElement, Opcode, Num, Stack, Script};
use super::error::Error;
use primitive_types::U256;

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

pub fn check_multiple_signature(public_keys: Vec<Vec<u8>>, signatures: Vec<Vec<u8>>, index: usize, z_privoder: &Box<dyn ZProvider>) -> Result<bool, Error>  {
    let mut pks = Vec::new();
    for public_key in public_keys {
        let pk = S256Point::parse(&public_key).map_err(|_| Error::InvalidPublicKey)?;
        pks.push(pk);
    }

    let mut z = U256::zero();
    let mut sighash: Option<SigHash> = None;
    let mut sigs = Vec::new();
    for sig_raw in signatures {
        let (sig, used) = Signature::parse_der(&sig_raw).map_err(|_| Error::InvalidSignature)?;
        sigs.push(sig);

        let sighash_now = if used + 1 == sig_raw.len() {
            SigHash::parse(sig_raw[used])?
        } else {
            SigHash::All // default is all
        };
        match sighash {
            None => {
                sighash = Some(sighash_now);
                z = z_privoder.z_u256(index, sighash_now)?;
            },
            Some(sighash) => {
                if sighash != sighash_now {
                    return Err(Error::SigHashIsNotTheSame);
                }
            }
        }
    }

    let mut correct_count = 0;
    for pk in &pks {
        for sig in &sigs {
            if sig.verify(z, pk.clone()) {
                correct_count += 1;
                break;
            }
        }
    }

    Ok(correct_count == sigs.len())
}

pub fn evaluate_p2sh(cmds: &mut Vec<CommandElement>, stack: &mut Stack, hash160: &Vec<u8>) -> Result<bool, Error> {
    println!(">> hash160: {}", hex::encode(hash160));
    let hash160_expect = hash::convert_slice_into_hash160(hash160);
    println!(">> hash160_expect: {}", hex::encode(&hash160_expect));

    let redeem_script_raw = stack.pop()?;
    println!(">> redeem_script_raw: {}", hex::encode(&redeem_script_raw));
    let hash160_real = hash::hash160(&redeem_script_raw);
    println!(">> hash160_real: {}", hex::encode(&hash160_real));
    if hash160_expect != hash160_real {
        println!("hash160 is not same in p2sh");
        return Ok(false);
    }

    println!("script parse start:");
    let (redeem_script, _used) = Script::parse(&redeem_script_raw)?;
    println!("script parse success: {:?}", redeem_script);
    cmds.clear();
    cmds.append(&mut redeem_script.cmds().clone());

    Ok(true)
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
        Opcode::OpCheckmultisig => {
            let (n, _) = varint::decode(&stack.pop()?)?;
            if n > 20 {
                return Err(Error::PublicKeyIsTooMuchForCheckMultisig);
            }

            let mut public_keys = Vec::new();
            for _ in 0..n {
                public_keys.push(stack.pop()?);
            }

            let (m, _) = varint::decode(&stack.pop()?)?;
            let mut signatures = Vec::new();
            for _ in 0..m {
                signatures.push(stack.pop()?);
            }

            stack.pop()?; // fix satoshi bug

            let result = check_multiple_signature(public_keys, signatures, index, z_privoder)?;
            let stack_result = if result { vec![1] } else { vec![] };
            stack.push(stack_result);
        }
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
