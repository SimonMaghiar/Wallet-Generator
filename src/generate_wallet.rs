use bip39::{Language, Mnemonic, MnemonicType, Seed};
use bitcoin::{
    secp256k1::Secp256k1,
    util::bip32::{DerivationPath, ExtendedPrivKey, ExtendedPubKey},
    Network, Address, PublicKey,
};
use crate::store_wallet::store_wallet_info;

pub fn generate_wallet (recovery_phrase_length: u16) -> (Mnemonic) {
    let (mnemonic, seed) = get_seed(recovery_phrase_length);

    let is_testnet = false; // Change this to `false` for mainnet
    let network = if is_testnet { "testnet" } else { "mainnet" };
    println!("Network: {}", network);

    // Generate the root key from the seed (using Bitcoin network type as a placeholder)
    let root_key = ExtendedPrivKey::new_master(bitcoin::Network::Bitcoin, seed.as_bytes()).expect("Valid seed");

    // Display the master private key (For demonstration purposes; do not do this in production)
    println!("Master Private Key: 0x{}",hex::encode(root_key.to_priv().to_bytes()));

    let derivation_path = if is_testnet {
        "m/44'/1'/0'/0/0" // BIP44 path for mainnet
    } else {
        "m/44'/2'/0'/0/0" // BIP44 path for testnet
    };
    store_wallet_info(derivation_path, &root_key);
    return mnemonic;
}

fn get_seed(recovery_phrase_length: u16) -> (Mnemonic, Seed) {
    let mnemonic: Mnemonic;
    if recovery_phrase_length == 12 {
        mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
    } else {
        mnemonic = Mnemonic::new(MnemonicType::Words24, Language::English);
    }
    // Generate a new mnemonic phrase
    println!("Generated Mnemonic: {:?}", mnemonic);

    // Derive the seed from the mnemonic phrase
    let seed = Seed::new(&mnemonic, "");
    (mnemonic, seed)
}
