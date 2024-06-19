pub fn generate_wallet_address (derived_priv_key: ExtendedPrivKey, is_testnet: bool) {
    let ctx = Secp256k1::new();
    // Derive the extended public key from the derived private key
    let derived_pub_key = ExtendedPubKey::from_priv(&ctx, &derived_priv_key);

    // Display the extended public key (For demonstration purposes; adjust for your cryptocurrency)
    // println!("Extended Public Key: {}", derived_pub_key);

    // Derive the public key from the extended public key
    let public_key = derived_pub_key
        .derive_pub(
            &ctx,
            &"m/0/0".parse::<DerivationPath>().expect("Valid path"),
        )
        .expect("Valid derivation")
        .to_pub();

    // Display the public key (For demonstration purposes; do not do this in production)
    // println!("Public Key: 0x{}",hex::encode(public_key.to_bytes()));
    let address = generate_address(public_key, is_testnet);
    // println!("Address: {}", address);
}

pub fn generate_address(pub_key: PublicKey, is_testnet: bool) -> String {
    let network = if is_testnet {
        Network::Testnet
    } else {
        Network::Bitcoin
    };

    let address = Address::p2pkh(&pub_key, network);

    address.to_string()
}