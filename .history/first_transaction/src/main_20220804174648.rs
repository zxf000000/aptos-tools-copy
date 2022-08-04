// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use first_transaction::{Account, FaucetClient, RestClient, FAUCET_URL, TESTNET_URL};
use ed25519_dalek::SecretKey;

//:!:>section_7
fn main() -> () {

    SecretKey.from_bytes("b4dac22ba2652a1c99ec21809da28d1da4e1423bb6eb68969a8e5ec28a610f1c")

    let rest_client = RestClient::new(TESTNET_URL.to_string());
    let faucet_client = FaucetClient::new(FAUCET_URL.to_string(), rest_client.clone());

    // Create two accounts, Alice and Bob, and fund Alice but not Bob
    let mut alice = Account::new(None);
    let bob = Account::new(None);

    println!("\n=== Addresses ===");
    println!("Alice: 0x{}", alice.address());
    println!("Bob: 0x{}", bob.address());

    faucet_client.fund_account(&alice.auth_key().as_str(), 5_000);
    faucet_client.fund_account(&bob.auth_key().as_str(), 0);

    println!("\n=== Initial Balances ===");
    println!("Alice: {:?}", rest_client.account_balance(&alice.address()));
    println!("Bob: {:?}", rest_client.account_balance(&bob.address()));

    // Have Alice give Bob 10 coins
    let tx_hash = rest_client.transfer(&mut alice, &bob.address(), 1_000);
    rest_client.wait_for_transaction(&tx_hash);

    println!("\n=== Final Balances ===");
    println!("Alice: {:?}", rest_client.account_balance(&alice.address()));
    println!("Bob: {:?}", rest_client.account_balance(&bob.address()));
}
//<:!:section_7
