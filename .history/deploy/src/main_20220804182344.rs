// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use deploy::HelloBlockchainClient;
use ed25519_dalek::{ExpandedSecretKey, SecretKey};
use first_transaction::{Account, FaucetClient, FAUCET_URL, TESTNET_URL};
use std::env;
fn main() -> () {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    assert_eq!(
        args.len(),
        1,
        "Expecting an argument that points to the helloblockchain module"
    );

    let pkey = [
        100, 183, 240, 129, 232, 57, 8, 53, 98, 150, 55, 54, 79, 11, 196, 108, 255, 176, 106, 109,
        18, 204, 198, 4, 202, 170, 188, 204, 25, 149, 29, 189,
    ];
    let address = "5bd848752e1d465dba3b301446d0f649acedcb9d36fe5c463b49ff869c819b57";

    let exp = SecretKey::from_bytes(&pkey).unwrap();
    let mut account = Account::new(Some(&pkey.));

    println!("account: {}", account.address());

    // let client = HelloBlockchainClient::new(TESTNET_URL.to_string());
    // let faucet_client = FaucetClient::new(FAUCET_URL.to_string(), client.rest_client.clone());

    // faucet_client.fund_account(address, 5_000);

    // let module_path = args.get(1).unwrap();
    // let module_hex = hex::encode(std::fs::read(module_path).unwrap());

    // println!("\n=== Testing Alice ===");
    // println!("Publishing...");
    // let mut tx_hash = client.publish_module(&mut account, &module_hex);
    // client.rest_client.wait_for_transaction(&tx_hash);
    // println!("Success");
}
