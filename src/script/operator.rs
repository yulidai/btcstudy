use crate::secp256k1::{S256Point, Signature};
use crate::transaction::{Transaction, TxOut, SigHash};
use crate::util::{hash, varint};
use super::{CommandElement, Opcode, Num, Stack, Script, ScriptBuilder, ZProvider, TransactionLegacyZProvider, TransactionWitnessP2pkhZProvider};
use super::error::Error;
use primitive_types::U256;

pub fn verify_tx(tx: &Transaction) -> Result<bool, Error> {
    let z_provider = Box::new(TransactionLegacyZProvider::from(tx.clone())) as Box<dyn ZProvider>;

    let mut amount_in = 0;
    for (i, input) in tx.inputs.iter().enumerate() {
        let output_ref = input.get_output_ref()?;
        let combined_script = Script::parse_raw(&output_ref.script())? + Script::parse_raw(&input.script)?;
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

pub fn verify_tx_input(tx: &Transaction, input_index: usize, prevout: Option<TxOut>) -> Result<bool, Error> {
    let input = &tx.inputs[input_index]; // TODO check len
    let prevout = match prevout {
        Some(prevout) => prevout,
        None => input.get_output_ref()?
    };

    let script_pubkey = Script::parse_raw(prevout.script())?;
    let script_sig = Script::parse_raw(&input.script)?;
    let (provider, combined_script) = if script_pubkey.is_p2wpkh_pubkey() && script_sig.is_empty() {
        let mut provider = TransactionWitnessP2pkhZProvider::from(tx.clone());
        let cache_key = [input.prev_tx.to_vec(), input.prev_index.serialize().to_vec()].concat();
        provider.prevout_cache.insert(cache_key, prevout);
        let provider = Box::new(provider) as Box<dyn ZProvider>;

        let pk_hash = hash::convert_slice_into_hash160(&script_pubkey.get_bottom_as_data().unwrap());
        let script_pubkey = ScriptBuilder::p2pkh(&pk_hash);
        let script_witness = Script::parse_witness(&input.witness).unwrap();

        (provider, script_pubkey + script_witness)
    } else {
        let provider = TransactionLegacyZProvider::from(tx.clone());
        let provider = Box::new(provider) as Box<dyn ZProvider>;
        (provider, script_pubkey + script_sig)
    };

    Ok(combined_script.evaluate(input_index, &provider).unwrap())
}

pub fn check_signature(pk: Vec<u8>, sig_raw: Vec<u8>, index: usize, z_privoder: &Box<dyn ZProvider>) -> Result<bool, Error>  {
    let pk = S256Point::parse(&pk).map_err(|_| Error::InvalidPublicKey)?;
    let (sig, used) = Signature::parse_der(&sig_raw).map_err(|_| Error::InvalidSignature)?;

    let sighash = if used + 1 == sig_raw.len() {
        SigHash::parse(sig_raw[used])?
    } else {
        SigHash::All // default is all
    };
    let z = z_privoder.z_u256(index, sighash, None, None)?;

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
                z = z_privoder.z_u256(index, sighash_now, None, None)?;
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
    let hash160_expect = hash::convert_slice_into_hash160(hash160);
    let redeem_script_raw = stack.pop()?;
    let hash160_real = hash::hash160(&redeem_script_raw);
    if hash160_expect != hash160_real {
        return Ok(false);
    }

    let redeem_script = Script::parse(&redeem_script_raw)?;
    cmds.clear();
    cmds.append(&mut redeem_script.cmds().clone());

    Ok(true)
}

pub fn evaluate_command(cmd: CommandElement, stack: &mut Stack, index: usize, z_privoder: &Box<dyn ZProvider>) -> Result<bool, Error> {
    let mut result = true;
    match cmd {
        CommandElement::Op(op) => result = evaluate_opcode(op, stack, index, z_privoder)?,
        CommandElement::Data(data) => stack.push(data),
        CommandElement::Unknown(byte) => return Err(Error::UnknownByteInScript(byte)),
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
    use crate::transaction::{Transaction, TxOut};

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

    #[test]
    fn operator_verify_transaction_input_true() {
        let bytes = hex::decode("01000000000102fff7f7881a8099afa6940d42d1e7f6362bec38171ea3edf433541db4e4ad969f00000000494830450221008b9d1dc26ba6a9cb62127b02742fa9d754cd3bebf337f7a55d114c8e5cdd30be022040529b194ba3f9281a99f2b1c0a19c0489bc22ede944ccf4ecbab4cc618ef3ed01eeffffffef51e1b804cc89d182d279655c3aa89e815b1b309fe287d9b2b55d57b90ec68a0100000000ffffffff02202cb206000000001976a9148280b37df378db99f66f85c95a783a76ac7a6d5988ac9093510d000000001976a9143bde42dbee7e4dbe6a21b2d50ce2f0167faa815988ac000247304402203609e17b84f6a7d30c80bfa610b5b4542f32a8a0d5447a12fb1366d7f01cc44a0220573a954c4518331561406f90300e8f3358f51928d43c212a8caed02de67eebee0121025476c2e83188368da1ff3e292e7acafcdb3566bb0ad253f62fc70f07aeee635711000000").unwrap();
        let tx = Transaction::parse(&bytes).unwrap();

        let prevout_bytes = hex::decode("0046c323000000001600141d0f172a0ecb48aee1be1f2687d2963ae33f71a1").unwrap();
        let prevout = TxOut::parse(&prevout_bytes).unwrap();

        assert_eq!(super::verify_tx_input(&tx, 1, Some(prevout)).unwrap(), true);
    }

    #[test]
    fn operator_verify_transaction_input_false() {
        let bytes = hex::decode("01000000000102fff7f7881a8099afa6940d42d1e7f6362bec38171ea3edf433541db4e4ad969f00000000494830450221008b9d1dc26ba6a9cb62127b02742fa9d754cd3bebf337f7a55d114c8e5cdd30be022040529b194ba3f9281a99f2b1c0a19c0489bc22ede944ccf4ecbab4cc618ef3ed01eeffffffef51e1b804cc89d182d279655c3aa89e815b1b309fe287d9b2b55d57b90ec68a0100000000ffffffff02202cb206000000001976a9148280b37df378db99f66f85c95a783a76ac7a6d5988ac9093510d000000001976a9143bde42dbee7e4dbe6a21b2d50ce2f0167faa815988ac0002483045022037206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c60221008ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec0121025476c2e83188368da1ff3e292e7acafcdb3566bb0ad253f62fc70f07aeee635711000000").unwrap();
        let tx = Transaction::parse(&bytes).unwrap();

        let prevout_bytes = hex::decode("0046c323000000001600141d0f172a0ecb48aee1be1f2687d2963ae33f71a1").unwrap();
        let prevout = TxOut::parse(&prevout_bytes).unwrap();

        assert_eq!(super::verify_tx_input(&tx, 1, Some(prevout)).unwrap(), false);
    }
}
