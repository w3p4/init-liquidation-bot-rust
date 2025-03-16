use alloy::{
    network::EthereumWallet,
    primitives::U256,
    providers::ProviderBuilder,
    signers::local::{coins_bip39::English, MnemonicBuilder},
};

use eyre::Result;
use futures::future;
use std::env;

mod addresses;
mod liquidation;
mod positions;
mod routers;
mod tokens;

const RPC_URL: &str = "https://rpc.mantle.xyz";

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "loop" {
        println!("loop!");
        loop {
            let _ = fetch_and_liquidate().await;
        }
    } else {
        let _ = fetch_and_liquidate().await;
    }
}

async fn fetch_and_liquidate() -> Result<(), Box<dyn std::error::Error>> {
    let phrase = env::var("PHRASE").expect("PHRASE must be set");
    let profit = env::var("PROFIT").expect("PROFIT must be set");
    let signer_number = env::var("SIGNER_NUMBER").expect("SIGNER_NUMBER must be set");

    // Instantiate a mnemonic signer.
    let mnemonic_signers = MnemonicBuilder::<English>::default().phrase(phrase);

    // create wallet providers
    let mut wallets: Vec<EthereumWallet> = Vec::new();
    for i in 0..signer_number.parse::<u32>().unwrap() {
        let mnemonic_signer = mnemonic_signers.clone().index(i)?.build()?;
        let wallet = EthereumWallet::from(mnemonic_signer);
        wallets.push(wallet);
    }
    let providers = future::try_join_all(
        wallets.iter().map(|w| ProviderBuilder::new().wallet(w.clone()).connect(RPC_URL)),
    )
    .await?;

    // get unhealth-active position infos
    let pos = positions::get_or_fetch_active_positions().await?;
    let pos_infos =
        positions::get_int_pos_infos_chunk(providers[0].clone(), pos.clone(), 150).await?;

    // logs
    let len = &pos_infos.len();
    println!("Unhealthy position: {len}");
    let active_pos_ids = pos_infos.iter().map(|pos| pos.posId).collect::<Vec<U256>>();

    let mut succeed = 0;
    let mut failed = 0;

    // try to liquidate
    // TODO: use 3 wallets to liquidate
    for (i, pos_id) in active_pos_ids.iter().enumerate() {
        let result = liquidation::try_liquidate(providers[0].clone(), pos_id, &profit).await;
        match result {
            Ok(()) => succeed += 1,
            Err(_error) => failed += 1,
        };
        print!("liquidated {}/{}, succeed: {}, failed: {}\r", i, len, succeed, failed);
    }
    Ok(())
}
