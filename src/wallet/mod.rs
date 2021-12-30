//TODO impl bip39, bip32 and so on

#[cfg(test)]
mod test {
    use crate::{
        transaction::{Transaction, TxIn, TxOut, PrevIndex, Sequence, Version, LockTime},
        util::{hash, base58},
        script::ScriptBuilder,
    };

    #[test]
    fn wallet_create_transaction() {
        let prev_tx = hash::convert_slice_into_hash256(&hex::decode("0d6fe5213c0b3291f208cba8bfb59b7476dffacc4e5cb66f6eb20a080843a299").unwrap());
        let prev_index = PrevIndex::new(13);
        let script = vec![];
        let sequence = Sequence::parse(&[0xffu8; 4]).unwrap();
        let witness = Vec::new();
        let tx_in = TxIn { prev_tx, prev_index, script, sequence, witness };

        let change_amount = 33000000u64;
        let change_h160 = base58::decode_btc_addr("mzx5YhAH9kNHtcN481u6WkjeHjYtVeKVh2").unwrap();
        let change_h160 = hash::convert_slice_into_hash160(&change_h160[1..]); // skip network
        let change_script = ScriptBuilder::p2pkh(&change_h160).raw_serialize().unwrap();
        let change_output = TxOut::new(change_amount, change_script);

        let target_amount = 10000000u64;
        let target_h160 = base58::decode_btc_addr("mnrVtF8DWjMu839VW3rBfgYaAfKk8983Xf").unwrap();
        let target_h160 = hash::convert_slice_into_hash160(&target_h160[1..]); // skip network
        let target_script = ScriptBuilder::p2pkh(&target_h160).raw_serialize().unwrap();
        let target_output = TxOut::new(target_amount, target_script);

        let version = Version::new(1);
        let inputs = vec![tx_in];
        let outputs = vec![change_output, target_output];
        let locktime = LockTime::new(0);
        let segwit = None;
        let transaction = Transaction { version, inputs, outputs, locktime, segwit };

        assert_eq!("cd30a8da777d28ef0e61efe68a9f7c559c1d3e5bcd7b265c850ccb4068598d11", hex::encode(transaction.id().unwrap()));

        // // sign transaction(test in the future)
        // let z = transaction.z_with_default_script(0, SigHash::All, change_script).unwrap();
        // let z = U256::from_big_endian(&z);
        // let priv_key = PrivateKey::new(8675309.into()).unwrap();
        // let signature = priv_key.sign_deterministic(z).unwrap();

        // let mut sig = hex::decode(signature.der()).unwrap();
        // sig.push(SigHash::All.value());
        // let sig = CommandElement::Data(sig);

        // let sec = priv_key.pk_point().sec_compressed().unwrap();
        // let sec = CommandElement::Data(sec);

        // let script = Script::new(vec![sig, sec]);
        // transaction.inputs[0].script = script;

        // println!("tx from wallet: {}", hex::encode(transaction.serialize().unwrap()));
    }

    // #[test]
    // fn sign_transaction() {
    //     let tx_bytes = hex::decode("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006a47304402207db2402a3311a3b845b038885e3dd889c08126a8570f26a844e3e4049c482a11022010178cdca4129eacbeab7c44648bf5ac1f9cac217cd609d216ec2ebc8d242c0a012103935581e52c354cd2f484fe8ed83af7a3097005b2f9c60bff71d35bd795f54b67feffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600").unwrap();
    //     let (tx, _) = Transaction::parse(&tx_bytes).unwrap();
    //     println!("tx: {:?}", tx);
    //     println!("tx_serialize: {}", hex::encode(tx.serialize().unwrap()));

        // let priv_key = PrivateKey::new(8675309.into());
        // let z = tx.z(SigHash::All);

    // }
}

