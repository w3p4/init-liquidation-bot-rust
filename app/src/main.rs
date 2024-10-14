use alloy::{
    primitives::{uint, Address},
    providers::{builder, Provider},
    transports::http::{Client, Http},
};
// use alloy_primitives::ruint::UintTryFrom;
use eyre::Result;
use foundry_contracts::iinitlens::IInitLens::{self, IInitLensInstance};

#[tokio::main]
async fn main() -> Result<()> {
    const INIT_LENS_ADDRESS: &str = "0x4403F4296BeF042a08785077D67F4700478800C5";

    let provider = builder().with_recommended_fillers().on_anvil_with_wallet();

    // let address = Address::parse_checksummed(checksummed, None).expect("valid checksum");
    // struct Test {
    //     c: &'a IInitLens::IInitLensInstance<Http<Client>, _>,
    // }

    let init_lens: IInitLensInstance<Http<Client>, _> =
        IInitLens::new(INIT_LENS_ADDRESS.parse::<Address>()?, provider.clone());
    // let test = Test { c: &init_lens };

    let address = init_lens.address();

    let builder = init_lens.getInitPosInfo(uint!(1234_U256));
    // let data = builder.call().await?;

    // println!("hello {}", address);
    // let blk = provider.get_block_number().await?;
    Ok(())
}
