use bip39::{Language, Mnemonic, Seed};
use bitcoin::{
    secp256k1::Secp256k1,
    util::bip32::{DerivationPath, ExtendedPrivKey, ExtendedPubKey},
    Network, Address, PublicKey,
};

use crate::store_wallet::store_wallet_info;

pub fn recover_wallet(recovery_phrase: &str) -> bool {
    let (mnemonic, seed) = get_seed(recovery_phrase);

    // Choose the network: mainnet or testnet
    let is_testnet = false; // Change this to `false` for mainnet
    let network = if is_testnet { "testnet" } else { "mainnet" };
    println!("Network: {}", network);

    // Generate the root key from the seed (using Bitcoin network type as a placeholder)
    let root_key = ExtendedPrivKey::new_master(bitcoin::Network::Bitcoin, seed.as_bytes()).expect("Valid seed");


    // Display the master private key (For demonstration purposes; do not do this in production)
    println!("Master Private Key: 0x{}",hex::encode(root_key.to_priv().to_bytes()));


    // Derive the private key at the custom derivation path for TrainCoin
    let derivation_path = if is_testnet {
        "m/44'/1'/0'/0/0" // BIP44 path for testnet
    } else {
        "m/44'/2'/0'/0/0" // BIP44 path for mainnet
    };
    store_wallet_info(derivation_path, &root_key);
    return true;
}

fn get_seed(mnemonic_phrase: &str) -> (Mnemonic, Seed) {
    // Generate a new mnemonic phrase
    // let mnemonic_phrase = "reward off answer drill install absent project hammer three wall ill supreme";
    let mnemonic = Mnemonic::from_phrase(mnemonic_phrase, Language::English).expect("Valid mnemonic");
    let seed = Seed::new(&mnemonic, "");
    (mnemonic, seed)
}