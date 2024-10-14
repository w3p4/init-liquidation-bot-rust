use alloy::{
    primitives::{uint, utils::format_units, Address},
    providers::ProviderBuilder,
    transports::http::{Client, Http},
};
use eyre::Result;
use foundry_contracts::iinitlens::IInitLens::{self, IInitLensInstance};

#[tokio::main]
async fn main() -> Result<()> {
    // constants
    const INIT_LENS_ADDRESS: &str = "0x4403F4296BeF042a08785077D67F4700478800C5";
    const _INIT_CORE: &str = "0x972BcB0284cca0152527c4f70f8F689852bCAFc5";
    const _POS_MANAGER: &str = "0x0e7401707CD08c03CDb53DAEF3295DDFb68BBa92";
    const _SWAP_DATA_REGISTRY: &str = "0x94670598E98f8DAd95D85932dD85CBD050CE1402";

    // Spin up a forked Anvil node.
    // Ensure `anvil` is available in $PATH.
    let rpc_url = "https://rpc.mantle.xyz";
    let provider =
        ProviderBuilder::new().on_anvil_with_wallet_and_config(|anvil| anvil.fork(rpc_url));

    // init lens instance
    let init_lens: IInitLensInstance<Http<Client>, _> =
        IInitLens::new(INIT_LENS_ADDRESS.parse::<Address>()?, provider.clone());

    // example pos id
    let pos_id =
        uint!(13896239034349855609814759822748684436256448872554724177418048799807749705170_U256);

    // create builder
    let builder = init_lens.getInitPosInfo(pos_id);

    // call
    let data = builder.call().await?;

    // destruct return data
    let info = data.posInfo;
    let health = info.health_e18;
    let health_string: String = format_units(health, 18)?;

    // print Info
    println!("health: {health_string}");
    Ok(())
}
