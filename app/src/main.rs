use alloy::{
    network::EthereumWallet, primitives::U256, providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
};
use dotenv::dotenv;
use eyre::Result;
use reqwest::Url;
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
    dotenv().ok(); // Reads the .env file
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    let profit = env::var("PROFIT").expect("PROFIT must be set");

    let url = Url::parse(RPC_URL)?;
    // Instantiate a signer.
    let signer: PrivateKeySigner = private_key.parse().expect("should parse private key");
    let wallet = EthereumWallet::from(signer);

    // let wallet = WalletProvider::new().with_signer(signer).on_http(url);
    let provider = ProviderBuilder::new().wallet(wallet).on_http(url);

    // get unhealth-active position infos
    let pos = positions::get_or_fetch_active_positions().await?;
    let pos_infos = positions::get_int_pos_infos_chunk(provider.clone(), pos.clone(), 150).await?;

    // logs
    let len = &pos_infos.len();
    println!("Unhealthy position: {len}");
    let active_pos_ids = pos_infos.iter().map(|pos| pos.posId).collect::<Vec<U256>>();

    let mut succeed = 0;
    let mut failed = 0;

    // try to liquidate
    for (i, pos_id) in active_pos_ids.iter().enumerate() {
        let result = liquidation::try_liquidate(provider.clone(), pos_id, &profit).await;
        match result {
            Ok(()) => succeed += 1,
            Err(_error) => failed += 1,
        };
        print!("liquidated {}/{}, succeed: {}, failed: {}\r", i, len, succeed, failed);
    }
    Ok(())
}
