use serde::{Deserialize, Serialize};
use serde_json::Number;
use std::collections::HashMap;

use alloy::{
    primitives::{utils::format_units, Address},
    providers::{Provider, ProviderBuilder},
    transports::http::{Client, Http},
};

use alloy::primitives::{Address, U256};

use foundry_contracts::iinitlens::IInitLens::{self, IInitLensInstance};

const BACKEND_API: &str = "https://index.init.capital/positions/positions";
const INIT_LENS_ADDRESS: &str = "0x4403F4296BeF042a08785077D67F4700478800C5";

#[derive(Serialize, Deserialize)]
pub struct RawData {
    borrow_pool_tokens: HashMap<String, String>,
    collateral_pool_tokens: HashMap<String, String>,
    owner: String,
    viewer: String,
}

#[derive(Serialize, Deserialize)]
struct RawPositions {
    data: HashMap<String, RawData>,
    status_code: Number,
}

#[derive(Debug)]
pub struct Position {
    pub pos_id: U256,
    pub borrow_pool_tokens: Vec<Address>,
    pub collateral_pool_tokens: Vec<Address>,
}

pub async fn get_active_positions() -> Result<Vec<Position>, Box<dyn std::error::Error>> {
    let resp: RawPositions = reqwest::get(BACKEND_API).await?.json::<RawPositions>().await?;

    let mut positions = Vec::<Position>::new();

    for (key, value) in resp.data.iter() {
        positions.push(Position {
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
        });
    }

    // filter only active positions
    let filtered_positions = positions
        .into_iter()
        .filter(|pos| pos.borrow_pool_tokens.len() > 0 && pos.collateral_pool_tokens.len() > 0)
        .collect::<Vec<Position>>();

    // println!("{:#?}", filtered_positions);

    Ok(filtered_positions)
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
