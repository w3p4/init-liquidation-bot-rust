use alloy::{primitives::utils::format_units, providers::ProviderBuilder};
use reqwest::Url;

use eyre::Result;

mod positions;

const RPC_URL: &str = "https://rpc.mantle.xyz";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse(RPC_URL)?;
    let provider = ProviderBuilder::new().with_recommended_fillers().on_http(url);

    // get active positions
    let pos = positions::get_active_position_ids().await?;

    let pos = pos.to_vec();
    let pos_infos = positions::get_int_pos_infos_chunk(provider, pos, 150).await?;

    let len = pos_infos.len();
    println!("Unhealthy position: {len}");
    // let health = pos_infos[0].health_e18;
    // let health_string: String = format_units(health, 18)?;
    // print Info
    // println!("health: {health_string}");
    Ok(())
}
