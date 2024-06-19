use std::fs::File;
use std::io::prelude::*;
use bip39::{Language, Mnemonic, MnemonicType, Seed};
use bitcoin::{
    secp256k1::Secp256k1,
    util::bip32::{DerivationPath, ExtendedPrivKey, ExtendedPubKey},
    Network, Address, PublicKey,
};

const FILENAME: &str = "wallet.dat";

pub fn store_wallet_info(derivation_path_base: &str, root_key: &ExtendedPrivKey) {
    // Create a new .dat file or overwrite existing
    let mut derivation_path = derivation_path_base.to_string();
    derivation_path.pop();
    derivation_path.pop();
    derivation_path.pop();
    derivation_path.pop();
    let mut file = File::create(FILENAME).expect("Failed to create file");
    let ctx = Secp256k1::new();
    // Generate 20 addresses for sending funds
    file.write_all(b"Sending/receiving addresses: \n").expect("Failed to write to file");
    for i in 0..20 {
        let derivation_path = format!("{}/{}/{}", derivation_path,"0",i);
        println!("derivation_path: {}", derivation_path);
        let derived_priv_key = root_key
        .derive_priv(
            &ctx,
            &derivation_path
                .parse::<DerivationPath>()
                .expect("Valid path"),
        )
        .expect("Valid derivation");
    println!("Derived Private Key: 0x{}",hex::encode(derived_priv_key.to_priv().to_bytes()));

    let priv_key_hex = hex::encode(derived_priv_key.to_priv().to_bytes());
    file.write_all(priv_key_hex.as_bytes()).expect("Failed to write to file");
    file.write_all(b"\n").expect("Failed to write newline to file");
    }
    // Generate 20 addresses for receiving funds (change address)
    file.write_all(b"\nChange addresses: \n").expect("Failed to write to file");
    for i in 0..20 {
        let derivation_path = format!("{}/{}/{}", derivation_path,"1",i);
        println!("derivation_path: {}", derivation_path);
        let derived_priv_key = root_key
        .derive_priv(
            &ctx,
            &derivation_path
                .parse::<DerivationPath>()
                .expect("Valid path"),
        )
        .expect("Valid derivation");
    println!("Derived Private Key: 0x{}",hex::encode(derived_priv_key.to_priv().to_bytes()));

    let priv_key_hex = hex::encode(derived_priv_key.to_priv().to_bytes());
    file.write_all(priv_key_hex.as_bytes()).expect("Failed to write to file");
    file.write_all(b"\n").expect("Failed to write newline to file");
    }
    println!("Wallet information stored in {}", FILENAME);
}
