use alloy::{primitives::utils::format_units, providers::ProviderBuilder};

use eyre::Result;

mod positions;

const RPC_URL: &str = "https://rpc.mantle.xyz";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Spin up a forked Anvil node.
    let provider =
        ProviderBuilder::new().on_anvil_with_wallet_and_config(|anvil| anvil.fork(RPC_URL));

    // get active positions
    let pos = positions::get_active_position_ids().await?;

    let pos = pos[0..50].to_vec();
    let pos_infos = positions::get_int_pos_infos_chunk(provider, pos, 150).await?;

    let len = pos_infos.len();
    println!("Unhealthy position: {len}");
    // let health = pos_infos[0].health_e18;
    // let health_string: String = format_units(health, 18)?;
    // print Info
    // println!("health: {health_string}");
    Ok(())
}
