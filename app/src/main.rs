use eyre::Result;
use reqwest::Url;

mod addresses;
mod positions;
mod routers;
mod tokens;
mod utils;

const RPC_URL: &str = "https://rpc.mantle.xyz";

use alloy::{
    network::EthereumWallet, primitives::U256, providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
};
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Reads the .env file
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set");
    let profit = env::var("PROFIT").expect("PROFIT must be set");

    let url = Url::parse(RPC_URL)?;
    // Instantiate a signer.
    let signer: PrivateKeySigner = private_key.parse().expect("should parse private key");
    let wallet = EthereumWallet::from(signer);

    // let wallet = WalletProvider::new().with_signer(signer).on_http(url);
    let provider = ProviderBuilder::new().with_recommended_fillers().wallet(wallet).on_http(url);

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
        let result = utils::try_liquidate(provider.clone(), pos_id, &profit).await;
        match result {
            Ok(()) => succeed += 1,
            Err(_error) => failed += 1,
        };
        print!("liquidated {}/{}, succeed: {}, failed: {}\r", i, len, succeed, failed);
    }
    Ok(())
}
