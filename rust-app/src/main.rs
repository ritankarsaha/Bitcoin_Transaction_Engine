use bitcoin::consensus::encode;
use bitcoin::hashes::Hash;
use bitcoin::key::{KeyPair, PublicKey};
use bitcoin::locktime::absolute::LockTime;
use bitcoin::sighash::{EcdsaSighashType, SighashCache};
use bitcoin::{
    Address, Amount, Network, OutPoint, PrivateKey, ScriptBuf, Sequence, Transaction, TxIn, TxOut,
    Witness,
};
use bitcoincore_rpc::{Auth, Client, RpcApi};
use secp256k1::{rand, Secp256k1};
use std::{thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Bitcoin transaction demo...");

    //  Connecting & loading the wallet
    let rpc = connect_to_bitcoin_node()?;

    //  Generating keypair & address
    let secp = Secp256k1::new();
    let kp = KeyPair::new(&secp, &mut rand::thread_rng());
    let privkey = PrivateKey::new(kp.secret_key().clone(), Network::Regtest);
    let pubkey = PublicKey::new(kp.public_key());
    let addr = Address::p2pkh(&pubkey, Network::Regtest);
    println!("Generated address: {}", addr);

    // Importing into legacy wallet
    println!("Importing private key...");
    rpc.import_private_key(&privkey, None, Some(false))?;

    //  Mining 101 blocks
    println!("Mining 101 blocks...");
    rpc.generate_to_address(101, &addr)?;

    // Grabbing the UTXO
    let utxo = rpc
        .list_unspent(Some(1), None, Some(&[&addr]), None, None)?
        .into_iter()
        .next()
        .expect("No UTXO found");
    println!("UTXO: {}:{} {}", utxo.txid, utxo.vout, utxo.amount);

    //  Building and submitting first tx
    let tx1 = create_transaction(&rpc, &kp, &secp, utxo.txid, utxo.vout as u32, utxo.amount, &addr, &pubkey)?;
    let tx1id = rpc.send_raw_transaction(&tx1)?;
    println!("First txid: {}", tx1id);

    rpc.generate_to_address(1, &addr)?;
    thread::sleep(Duration::from_secs(1));

    // Finding the UTXO from tx1
    let utxo2 = rpc
        .list_unspent(Some(1), None, Some(&[&addr]), None, None)?
        .into_iter()
        .find(|u| u.txid == tx1id)
        .expect("No UTXO from tx1");
    println!("UTXO2: {}:{} {}", utxo2.txid, utxo2.vout, utxo2.amount);

    // Building and submitting second tx
    let tx2 = create_transaction(&rpc, &kp, &secp, utxo2.txid, utxo2.vout as u32, utxo2.amount, &addr, &pubkey)?;
    let tx2id = rpc.send_raw_transaction(&tx2)?;
    println!("Second txid: {}", tx2id);

    rpc.generate_to_address(1, &addr)?;
    println!("Demo complete.");
    Ok(())
}

fn connect_to_bitcoin_node() -> Result<Client, bitcoincore_rpc::Error> {
    println!("Waiting for Bitcoin RPC...");
    thread::sleep(Duration::from_secs(5));

    let mut tries = 0;
    loop {
        match Client::new(
            "http://127.0.0.1:18443",
            Auth::UserPass("bitcoin".into(), "password".into()),
        ) {
            Ok(cli) if cli.get_blockchain_info().is_ok() => {
                println!("Connected to Bitcoin node.");
                if let Err(e) = cli.load_wallet("default") {
                    println!("Note: load_wallet error: {}", e);
                }
                return Ok(cli);
            }
            _ => {
                tries += 1;
                if tries > 30 {
                    panic!("Unable to reach Bitcoin RPC after 30 attempts");
                }
                println!("Retrying RPC… ({}/30)", tries);
                thread::sleep(Duration::from_secs(2));
            }
        }
    }
}

fn create_transaction(
    rpc: &Client,
    kp: &KeyPair,
    secp: &Secp256k1<secp256k1::All>,
    prev_txid: bitcoin::Txid,
    prev_vout: u32,
    amt: Amount,
    addr: &Address,
    pk: &PublicKey,
) -> Result<Transaction, Box<dyn std::error::Error>> {
    let fee = Amount::from_btc(0.00001)?;
    let send_amt = amt - fee;

    // preparing input/output
    let outp = OutPoint::new(prev_txid, prev_vout);
    let mut txin = TxIn {
        previous_output: outp,
        script_sig: ScriptBuf::new(),
        sequence: Sequence::MAX,
        witness: Witness::new(),
    };
    let txout = TxOut {
        value: send_amt.to_sat(),
        script_pubkey: addr.script_pubkey(),
    };
    let unsigned = Transaction {
        version: 2,
        lock_time: LockTime::ZERO,
        input: vec![txin.clone()],
        output: vec![txout.clone()],
    };

    //  RPC gives us raw bytes already—no hex decode needed
    let raw = rpc.get_transaction(&prev_txid, None)?.hex;
    let prev_tx: Transaction = encode::deserialize(&raw)?;

    let prev_script = prev_tx.output[prev_vout as usize].script_pubkey.clone();
    let cache = SighashCache::new(&unsigned);
    let sighash = cache.legacy_signature_hash(0, &prev_script, EcdsaSighashType::All.to_u32())?;
    let msg = secp256k1::Message::from_slice(&sighash.to_byte_array())?;
    let sig = secp.sign_ecdsa(&msg, &kp.secret_key());
    let mut der = sig.serialize_der().to_vec();
    der.push(EcdsaSighashType::All.to_u32() as u8);

    // building scriptSig
    let mut buf = Vec::with_capacity(der.len() + pk.to_bytes().len() + 2);
    buf.push(der.len() as u8);
    buf.extend_from_slice(&der);
    let pkb = pk.to_bytes();
    buf.push(pkb.len() as u8);
    buf.extend_from_slice(&pkb);
    txin.script_sig = ScriptBuf::from(buf);

    Ok(Transaction {
        version: 2,
        lock_time: LockTime::ZERO,
        input: vec![txin],
        output: vec![txout],
    })
}
