use alloy::providers::ProviderBuilder;
use eyre::Result;
use reqwest::Url;

mod addresses;
mod positions;
mod routers;

const RPC_URL: &str = "https://rpc.mantle.xyz";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse(RPC_URL)?;
    let provider = ProviderBuilder::new().with_recommended_fillers().on_http(url);

    // get unhealth-active position infos
    let pos = positions::get_or_fetch_active_positions().await?;
    let pos_infos = positions::get_int_pos_infos_chunk(provider, pos, 150).await?;

    // logs
    let len = pos_infos.len();
    println!("Unhealthy position: {len}");
    // let health = pos_infos[0].health_e18;
    // let health_string: String = format_units(health, 18)?;

    // TODO: try to liquidate
    Ok(())
}
