use bip39::{Language, Mnemonic, Seed};
use bitcoin::{
    secp256k1::Secp256k1,
    util::bip32::{DerivationPath, ExtendedPrivKey, ExtendedPubKey},
    Network, Address, PublicKey,
};

pub fn recover_wallet(recovery_phrase: &str) -> String {
    let (mnemonic, seed) = get_seed(recovery_phrase);

    // Choose the network: mainnet or testnet
    let is_testnet = false; // Change this to `false` for mainnet
    let network = if is_testnet { "testnet" } else { "mainnet" };
    println!("Network: {}", network);

    // Generate the root key from the seed (using Bitcoin network type as a placeholder)
    let root_key = ExtendedPrivKey::new_master(bitcoin::Network::Bitcoin, seed.as_bytes())
        .expect("Valid seed");


    // Display the master private key (For demonstration purposes; do not do this in production)
    println!(
        "Master Private Key: 0x{}",
        hex::encode(root_key.to_priv().to_bytes())
    );

    // Create a context for secp256k1 operations
    let ctx = Secp256k1::new();

    // Derive the private key at the custom derivation path for TrainCoin
    let derivation_path = if is_testnet {
        "m/44'/1'/0'/0/0" // BIP44 path for testnet
    } else {
        "m/44'/123'/0'/0/0" // BIP44 path for TrainCoin mainnet (replace '123' with your coin's BIP44 coin type)
    };
    let derived_priv_key = root_key
        .derive_priv(
            &ctx,
            &derivation_path
                .parse::<DerivationPath>()
                .expect("Valid path"),
        )
        .expect("Valid derivation");

    // Display the derived private key (For demonstration purposes; do not do this in production)
    println!(
        "Derived Private Key: 0x{}",
        hex::encode(derived_priv_key.to_priv().to_bytes())
    );

    // Derive the extended public key from the derived private key
    let derived_pub_key = ExtendedPubKey::from_priv(&ctx, &derived_priv_key);

    // Display the extended public key (For demonstration purposes; adjust for your cryptocurrency)
    println!("Extended Public Key: {}", derived_pub_key);

    // Derive the public key from the extended public key
    let public_key = derived_pub_key
        .derive_pub(
            &ctx,
            &"m/0/0".parse::<DerivationPath>().expect("Valid path"),
        )
        .expect("Valid derivation")
        .to_pub();

    // Display the public key (For demonstration purposes; do not do this in production)
    println!(
        "Public Key: 0x{}",
        hex::encode(public_key.to_bytes())
    );

    // Optionally, generate an address based on the network
    // This part depends on how addresses are formatted in TrainCoin
    let address = generate_address(public_key, is_testnet);
    println!("Address: {}", address);
    return hex::encode(public_key.to_bytes());
}

fn get_seed(mnemonic_phrase: &str) -> (Mnemonic, Seed) {
    // Generate a new mnemonic phrase
    // let mnemonic_phrase = "reward off answer drill install absent project hammer three wall ill supreme";
    let mnemonic = Mnemonic::from_phrase(mnemonic_phrase, Language::English).expect("Valid mnemonic");
    let seed = Seed::new(&mnemonic, "");
    (mnemonic, seed)
}

fn generate_address(pub_key: PublicKey, is_testnet: bool) -> String {
    let network = if is_testnet {
        Network::Testnet
    } else {
        Network::Bitcoin
    };

    let secp = Secp256k1::new();
    let address = Address::p2pkh(&pub_key, network);

    address.to_string()
}