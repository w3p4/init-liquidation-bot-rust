use alloy::{
    network::EthereumWallet,
    primitives::U256,
    providers::ProviderBuilder,
    signers::local::{coins_bip39::English, MnemonicBuilder},
};

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use tokio::sync::Semaphore;

use eyre::Result;
use futures::{future, StreamExt};

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
    let signer_number =
        env::var("SIGNER_NUMBER").expect("SIGNER_NUMBER must be set").parse::<usize>().unwrap();

    // Instantiate a mnemonic signer.
    let mnemonic_signers = MnemonicBuilder::<English>::default().phrase(phrase);

    // create wallet providers
    let mut wallets: Vec<EthereumWallet> = Vec::new();
    for i in 0..signer_number {
        let mnemonic_signer = mnemonic_signers.clone().index(i as u32)?.build()?;
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

    let succeed = Arc::new(AtomicU32::new(0));
    let failed = Arc::new(AtomicU32::new(0));

    let semaphore = Arc::new(Semaphore::new(signer_number));

    let result = futures::stream::iter(active_pos_ids.chunks(signer_number))
        .enumerate()
        .map(async |(_, chunk)| {
            let semaphore = Arc::clone(&semaphore);
            let _permit = semaphore.acquire().await?;

            let futures = chunk.iter().enumerate().map(|(i, pos_id)| {
                let provider = providers[i % providers.len()].clone();
                liquidation::try_liquidate(provider, pos_id, &profit)
            });

            let results = future::join_all(futures).await;

            for result in results {
                match result {
                    Ok(()) => {
                        succeed.fetch_add(1, Ordering::SeqCst);
                    }
                    Err(_) => {
                        failed.fetch_add(1, Ordering::SeqCst);
                    }
                };
            }

            print!("liquidated chunk, succeed: {:?}, failed: {:?}\r", succeed, failed);
            Ok::<(), Box<dyn std::error::Error>>(())
        })
        .buffer_unordered(4)
        .collect::<Vec<_>>()
        .await;
    println!("liquidated all positions, succeed: {:?}, failed: {:?}", succeed, failed);

    Ok(())
}
