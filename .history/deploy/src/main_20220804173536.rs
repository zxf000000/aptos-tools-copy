// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use first_transaction::{Account, FaucetClient, FAUCET_URL, TESTNET_URL};
use hello_blockchain::HelloBlockchainClient;
use std::env;

fn main() -> () {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    assert_eq!(
        args.len(),
        1,
        "Expecting an argument that points to the helloblockchain module"
    );

    let pkey = "0xb4dac22ba2652a1c99ec21809da28d1da4e1423bb6eb68969a8e5ec28a610f1c";
    let address = "5bd848752e1d465dba3b301446d0f649acedcb9d36fe5c463b49ff869c819b57";

    let account = Account::new(address.as_bytes)

    let client = HelloBlockchainClient::new(TESTNET_URL.to_string());
    let faucet_client = FaucetClient::new(FAUCET_URL.to_string(), client.rest_client.clone());

    faucet_client.fund_account(address, 5_000);

    let module_path = args.get(1).unwrap();
    let module_hex = hex::encode(std::fs::read(module_path).unwrap());

    println!("\n=== Testing Alice ===");
    println!("Publishing...");
    let mut tx_hash = client.publish_module(&mut address, &module_hex);
    client.rest_client.wait_for_transaction(&tx_hash);
    println!("Success");
}
