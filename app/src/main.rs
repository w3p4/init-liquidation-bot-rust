use alloy::{
    primitives::{uint, utils::format_units, Address},
    providers::ProviderBuilder,
    transports::http::{Client, Http},
};
// use alloy_primitives::ruint::UintTryFrom;
use eyre::Result;
use foundry_contracts::iinitlens::IInitLens::{self, IInitLensInstance};

#[tokio::main]
async fn main() -> Result<()> {
    const INIT_LENS_ADDRESS: &str = "0x4403F4296BeF042a08785077D67F4700478800C5";

    // Spin up a forked Anvil node.
    // Ensure `anvil` is available in $PATH.
    let rpc_url = "https://rpc.mantle.xyz";
    let provider =
        ProviderBuilder::new().on_anvil_with_wallet_and_config(|anvil| anvil.fork(rpc_url));

    let init_lens: IInitLensInstance<Http<Client>, _> =
        IInitLens::new(INIT_LENS_ADDRESS.parse::<Address>()?, provider.clone());

    let pos_id =
        uint!(13896239034349855609814759822748684436256448872554724177418048799807749705170_U256);

    let builder = init_lens.getInitPosInfo(pos_id);

    let data = builder.call().await?;
    let info = data.posInfo;
    let health = info.health_e18;

    let health_string: String = format_units(health, 18)?;

    println!("health: {health_string}");
    // let blk = provider.get_block_number().await?;
    Ok(())
}
