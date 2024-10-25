use serde::Deserialize;
use std::collections::HashMap;

use alloy::primitives::{Address, U256};

use foundry_contracts::iinitlens::IInitLens::{self, IInitLensInstance};

const BACKEND_API: &str = "https://index.init.capital/positions/positions";
const INIT_LENS_ADDRESS: &str = "0x4403F4296BeF042a08785077D67F4700478800C5";

#[derive(Deserialize)]
pub struct RawData {
    borrow_pool_tokens: HashMap<String, String>,
    collateral_pool_tokens: HashMap<String, String>,
}

#[derive(Deserialize)]
struct RawPositions {
    data: HashMap<String, RawData>,
    status_code: u32,
}

#[derive(Debug)]
pub struct Position {
    pub pos_id: U256,
    pub borrow_pool_tokens: Vec<Address>,
    pub collateral_pool_tokens: Vec<Address>,
}

pub async fn get_active_position_ids() -> Result<Vec<U256>, Box<dyn std::error::Error>> {
    let resp: RawPositions = reqwest::get(BACKEND_API).await?.json::<RawPositions>().await?;

    let mut filtered_position_ids = Vec::<U256>::new();

    if resp.status_code != 200 {
        println!("something wrong")
    };

    for (key, value) in resp.data.iter() {
        let pos = Position {
            pos_id: key.parse::<U256>()?,
            borrow_pool_tokens: value
                .borrow_pool_tokens
                .keys()
                .map(|x| x.parse::<Address>().unwrap())
                .collect::<Vec<Address>>(),
            collateral_pool_tokens: value
                .collateral_pool_tokens
                .keys()
                .map(|x| x.parse::<Address>().unwrap())
                .collect::<Vec<Address>>(),
        };
        // filtered only active position, add the position id to the list
        if !pos.borrow_pool_tokens.is_empty() && !pos.collateral_pool_tokens.is_empty() {
            filtered_position_ids.push(pos.pos_id)
        };
    }

    // println!("{:#?}", filtered_position_ids);
    Ok(filtered_position_ids)
}

// async fn get_init_pos_info(
//     provider: &ProviderBuilder>,
// ) -> Result<String, Box<dyn std::error::Error>> {
//     // init lens instance
//     let init_lens: IInitLensInstance<Http<Client>, _> =
//         IInitLens::new(INIT_LENS_ADDRESS.parse::<Address>()?, provider);

//     let builder = init_lens.getInitPosInfo(pos[0].pos_id);

//     // call
//     let data = builder.call().await?;

//     // destruct return data
//     let info = data.posInfo;
//     let health = info.health_e18;
//     let health_string: String = format_units(health, 18)?;
//     Ok(health_string)
// }
