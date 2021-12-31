use crate::secp256k1::{S256Point, Signature};
use crate::transaction::{Transaction, TxOut, SigHash};
use crate::util::{hash, varint};
use super::{CommandElement, Opcode, Num, Stack, Script, ScriptBuilder, ZProvider, TransactionLegacyZProvider, TransactionWitnessP2pkhZProvider};
use super::error::Error;
use primitive_types::U256;

pub fn verify_tx(tx: &Transaction) -> Result<bool, Error> {
    let mut z_provider = Box::new(TransactionLegacyZProvider::from(tx.clone())) as Box<dyn ZProvider>;

    let mut amount_in = 0;
    for (i, input) in tx.inputs.iter().enumerate() {
        let output_ref = input.get_output_ref()?;
        let combined_script = Script::parse_raw(&output_ref.script())? + Script::parse_raw(&input.script)?;
        if !combined_script.evaluate(i, &mut z_provider)? {
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

pub fn convert_script(tx: &Transaction, input_index: usize, prevout: Option<TxOut>) -> Result<(Script, Script, Box<dyn ZProvider>), Error> {
    let input = &tx.inputs[input_index]; // TODO check len
    let prevout = match prevout {
        Some(prevout) => prevout,
        None => input.get_output_ref()?
    };

    let mut script_pubkey = Script::parse_raw(prevout.script())?;
    let mut script_sig = Script::parse_raw(&input.script)?;
    let provider_box;

    // check is p2sh or not
    if script_pubkey.is_p2sh_pubkey() && script_sig.get_bottom_as_data().is_some() {
        let redeem_script = script_sig.get_bottom_as_data().unwrap();
        let redeem_hash_160 = &hash::hash160(&redeem_script)[..];
        let redeem_hash_expect = &script_pubkey.get_index_as_data(1).unwrap()[..];
        if redeem_hash_160 != redeem_hash_expect {
            return Err(Error::InvalidRedeemScript);
        }
        script_pubkey = Script::parse_raw(&redeem_script)?;

        let mut script_sig_cmds = script_sig.cmds().clone();
        script_sig_cmds.remove(0); // remove redeem script
        script_sig = Script::new(script_sig_cmds);
    }

    // check is witness or not
    if script_pubkey.is_p2wpkh_pubkey() && script_sig.is_empty() {
        let pk_hash = hash::convert_slice_into_hash160(&script_pubkey.get_bottom_as_data().unwrap());
        script_pubkey = ScriptBuilder::p2pkh(&pk_hash);
        script_sig = Script::parse_witness(&input.witness).unwrap(); // witness as sig

        let mut provider = TransactionWitnessP2pkhZProvider::from(tx.clone());
        let cache_key = [input.prev_tx.to_vec(), input.prev_index.serialize().to_vec()].concat();
        provider.prevout_cache.insert(cache_key, (prevout, script_pubkey.clone()));
        provider_box = Box::new(provider) as Box<dyn ZProvider>;
    } else if script_pubkey.is_p2wsh_pubkey() && script_sig.is_empty() {
        let script_witness = Script::parse_witness(&input.witness).unwrap();
        let redeem_script = script_witness.get_bottom_as_data().unwrap();
        let redeem_sha256 = &hash::sha256(&redeem_script)[..];
        let redeem_hash_expect = &script_pubkey.get_bottom_as_data().unwrap()[..];
        if redeem_sha256 != redeem_hash_expect {
            return Err(Error::InvalidWitnessRedeemScript);
        }
        script_pubkey = Script::parse_raw(&redeem_script)?;

        let mut script_sig_cmds = script_witness.cmds().clone();
        script_sig_cmds.remove(0); // remove redeem script
        script_sig = Script::new(script_sig_cmds);

        let mut provider = TransactionWitnessP2pkhZProvider::from(tx.clone());
        let cache_key = [input.prev_tx.to_vec(), input.prev_index.serialize().to_vec()].concat();
        provider.prevout_cache.insert(cache_key, (prevout, script_pubkey.clone()));
        provider_box = Box::new(provider) as Box<dyn ZProvider>;
    } else {
        let provider = TransactionLegacyZProvider::from(tx.clone());
        provider_box = Box::new(provider) as Box<dyn ZProvider>;
    }

    Ok((script_pubkey, script_sig, provider_box))
}

pub fn verify_tx_input(tx: &Transaction, input_index: usize, prevout: Option<TxOut>) -> Result<bool, Error> {
    let (script_pubkey, script_sig, mut provider) = convert_script(tx, input_index, prevout)?;
    let combined_script = script_pubkey + script_sig;

    Ok(combined_script.evaluate(input_index, &mut provider).unwrap())
}

pub fn check_signature(pk_raw: Vec<u8>, sig_raw: Vec<u8>, index: usize, z_privoder: &mut Box<dyn ZProvider>) -> Result<bool, Error>  {
    let pk = S256Point::parse(&pk_raw).map_err(|_| Error::InvalidPublicKey)?;
    let (sig, used) = Signature::parse_der(&sig_raw).map_err(|_| Error::InvalidSignature)?;

    let sighash = if used + 1 == sig_raw.len() {
        SigHash::parse(sig_raw[used])?
    } else {
        SigHash::All // default is all
    };
    let z = z_privoder.z_u256(index, sighash, None, None)?;

    Ok(sig.verify(z, pk))
}

pub fn check_multiple_signature(public_keys: Vec<Vec<u8>>, signatures: Vec<Vec<u8>>, index: usize, z_privoder: &mut Box<dyn ZProvider>) -> Result<bool, Error>  {
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

pub fn evaluate_command(cmd: CommandElement, stack: &mut Stack, index: usize, z_privoder: &mut Box<dyn ZProvider>) -> Result<bool, Error> {
    let mut result = true;
    match cmd {
        CommandElement::Op(op) => result = evaluate_opcode(op, stack, index, z_privoder)?,
        CommandElement::Data(data) => stack.push(data),
        CommandElement::Unknown(byte) => return Err(Error::UnknownByteInScript(byte)),
    };

    Ok(result)
}

fn evaluate_opcode(op: Opcode, stack: &mut Stack, index: usize, z_privoder: &mut Box<dyn ZProvider>) -> Result<bool, Error> {
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
        Opcode::OpVerify => {
            let ele = stack.pop()?;
            if Num::decode(ele)?.value() == 0 {
                return Ok(false);
            }
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
        Opcode::OpCodeseparator => {
            // nothing now
            // TODO make signature correct
        },
        Opcode::OpChecksigverify => {
            // op_checksig (TODO use function)
            let pk = stack.pop()?;
            let sig = stack.pop()?;
            result = check_signature(pk, sig, index, z_privoder)?;

            let stack_result = if result { vec![1] } else { vec![] };
            stack.push(stack_result);
            // op_verify (TODO use function)
            let ele = stack.pop()?;
            if Num::decode(ele)?.value() == 0 {
                return Ok(false);
            }
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
    fn operator_verify_transaction_input_p2wpkh_true() {
        let bytes = hex::decode("01000000000102fff7f7881a8099afa6940d42d1e7f6362bec38171ea3edf433541db4e4ad969f00000000494830450221008b9d1dc26ba6a9cb62127b02742fa9d754cd3bebf337f7a55d114c8e5cdd30be022040529b194ba3f9281a99f2b1c0a19c0489bc22ede944ccf4ecbab4cc618ef3ed01eeffffffef51e1b804cc89d182d279655c3aa89e815b1b309fe287d9b2b55d57b90ec68a0100000000ffffffff02202cb206000000001976a9148280b37df378db99f66f85c95a783a76ac7a6d5988ac9093510d000000001976a9143bde42dbee7e4dbe6a21b2d50ce2f0167faa815988ac000247304402203609e17b84f6a7d30c80bfa610b5b4542f32a8a0d5447a12fb1366d7f01cc44a0220573a954c4518331561406f90300e8f3358f51928d43c212a8caed02de67eebee0121025476c2e83188368da1ff3e292e7acafcdb3566bb0ad253f62fc70f07aeee635711000000").unwrap();
        let tx = Transaction::parse(&bytes).unwrap();

        let prevout_bytes = hex::decode("0046c323000000001600141d0f172a0ecb48aee1be1f2687d2963ae33f71a1").unwrap();
        let prevout = TxOut::parse(&prevout_bytes).unwrap();

        assert_eq!(super::verify_tx_input(&tx, 1, Some(prevout)).unwrap(), true);
    }

    #[test]
    fn operator_verify_transaction_input_p2wpkh_false() {
        let bytes = hex::decode("01000000000102fff7f7881a8099afa6940d42d1e7f6362bec38171ea3edf433541db4e4ad969f00000000494830450221008b9d1dc26ba6a9cb62127b02742fa9d754cd3bebf337f7a55d114c8e5cdd30be022040529b194ba3f9281a99f2b1c0a19c0489bc22ede944ccf4ecbab4cc618ef3ed01eeffffffef51e1b804cc89d182d279655c3aa89e815b1b309fe287d9b2b55d57b90ec68a0100000000ffffffff02202cb206000000001976a9148280b37df378db99f66f85c95a783a76ac7a6d5988ac9093510d000000001976a9143bde42dbee7e4dbe6a21b2d50ce2f0167faa815988ac0002483045022037206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c60221008ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec0121025476c2e83188368da1ff3e292e7acafcdb3566bb0ad253f62fc70f07aeee635711000000").unwrap();
        let tx = Transaction::parse(&bytes).unwrap();

        let prevout_bytes = hex::decode("0046c323000000001600141d0f172a0ecb48aee1be1f2687d2963ae33f71a1").unwrap();
        let prevout = TxOut::parse(&prevout_bytes).unwrap();

        assert_eq!(super::verify_tx_input(&tx, 1, Some(prevout)).unwrap(), false);
    }

    #[test]
    fn operator_verify_transaction_input_p2sh_p2wpkh_true() {
        let bytes = hex::decode("01000000000101db6b1b20aa0fd7b23880be2ecbd4a98130974cf4748fb66092ac4d3ceb1a5477010000001716001479091972186c449eb1ded22b78e40d009bdf0089feffffff02b8b4eb0b000000001976a914a457b684d7f0d539a46a45bbc043f35b59d0d96388ac0008af2f000000001976a914fd270b1ee6abcaea97fea7ad0402e8bd8ad6d77c88ac02473044022047ac8e878352d3ebbde1c94ce3a10d057c24175747116f8288e5d794d12d482f0220217f36a485cae903c713331d877c1f64677e3622ad4010726870540656fe9dcb012103ad1d8e89212f0b92c74d23bb710c00662ad1470198ac48c43f7d6f93a2a2687392040000").unwrap();
        let tx = Transaction::parse(&bytes).unwrap();

        let prevout = TxOut::new(1000000000u64, hex::decode("a9144733f37cf4db86fbc2efed2500b4f4e49f31202387").unwrap());

        assert_eq!(super::verify_tx_input(&tx, 0, Some(prevout)).unwrap(), true);
    }

    #[test]
    fn operator_verify_transaction_input_p2sh_p2wpkh_false() {
        let bytes = hex::decode("01000000000101db6b1b20aa0fd7b23880be2ecbd4a98130974cf4748fb66092ac4d3ceb1a5477010000001716001479091972186c449eb1ded22b78e40d009bdf0089feffffff02b8b4eb0b000000001976a914a457b684d7f0d539a46a45bbc043f35b59d0d96388ac0008af2f000000001976a914fd270b1ee6abcaea97fea7ad0402e8bd8ad6d77c88ac0247304402203609e17b84f6a7d30c80bfa610b5b4542f32a8a0d5447a12fb1366d7f01cc44a0220573a954c4518331561406f90300e8f3358f51928d43c212a8caed02de67eebee012103ad1d8e89212f0b92c74d23bb710c00662ad1470198ac48c43f7d6f93a2a2687392040000").unwrap();
        let tx = Transaction::parse(&bytes).unwrap();

        let prevout = TxOut::new(1000000000u64, hex::decode("a9144733f37cf4db86fbc2efed2500b4f4e49f31202387").unwrap());

        assert_eq!(super::verify_tx_input(&tx, 0, Some(prevout)).unwrap(), false);
    }

    #[test]
    fn operator_verify_transaction_input_p2wsh_true() {
        let bytes = hex::decode("01000000000102fe3dc9208094f3ffd12645477b3dc56f60ec4fa8e6f5d67c565d1c6b9216b36e000000004847304402200af4e47c9b9629dbecc21f73af989bdaa911f7e6f6c2e9394588a3aa68f81e9902204f3fcf6ade7e5abb1295b6774c8e0abd94ae62217367096bc02ee5e435b67da201ffffffff0815cf020f013ed6cf91d29f4202e8a58726b1ac6c79da47c23d1bee0a6925f80000000000ffffffff0100f2052a010000001976a914a30741f8145e5acadf23f751864167f32e0963f788ac000347304402200de66acf4527789bfda55fc5459e214fa6083f936b430a762c629656216805ac0220396f550692cd347171cbc1ef1f51e15282e837bb2b30860dc77c8f78bc8501e503473044022027dc95ad6b740fe5129e7e62a75dd00f291a2aeb1200b84b09d9e3789406b6c002201a9ecd315dd6a0e632ab20bbb98948bc0c6fb204f2c286963bb48517a7058e27034721026dccc749adc2a9d0d89497ac511f760f45c47dc5ed9cf352a58ac706453880aeadab210255a9626aebf5e29c0e6538428ba0d1dcf6ca98ffdf086aa8ced5e0d0215ea465ac00000000").unwrap();
        let tx = Transaction::parse(&bytes).unwrap();

        let prevout = TxOut::new(4900000000u64, hex::decode("00205d1b56b63d714eebe542309525f484b7e9d6f686b3781b6f61ef925d66d6f6a0").unwrap());

        assert_eq!(super::verify_tx_input(&tx, 1, Some(prevout)).unwrap(), true);
    }
}
