use serde::{Deserialize, Serialize};
use serde_json::Number;
use std::collections::HashMap;

use alloy::primitives::{Address, U256};

const BACKEND_API: &str = "https://index.init.capital/positions/positions";

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
