use futures::stream::StreamExt;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Semaphore;

use alloy::{
    contract::private::{Network, Provider, Transport},
    primitives::{Address, U256},
};

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

use thiserror::Error;
use tracing::{debug, error, info};

#[derive(Error, Debug)]
pub enum PositionError {
    #[error("API request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Failed to parse position ID: {0}")]
    ParseError(#[from] std::num::ParseIntError),

    #[error("API returned unexpected status code: {0}")]
    UnexpectedStatus(u32),
}

pub async fn get_active_position_ids() -> Result<Vec<U256>, PositionError> {
    println!("Fetching active positions from {}", BACKEND_API);

    let resp = reqwest::get(BACKEND_API).await?.json::<RawPositions>().await?;

    if resp.status_code != 200 {
        error!("API returned non-200 status code: {}", resp.status_code);
        return Err(PositionError::UnexpectedStatus(resp.status_code));
    }

    let filtered_position_ids: Vec<U256> = resp
        .data
        .iter()
        .filter_map(|(key, value)| {
            let pos_id = match key.parse::<U256>() {
                Ok(id) => id,
                Err(e) => {
                    error!("Failed to parse position ID {}: {}", key, e);
                    return None;
                }
            };

            let position = Position {
                pos_id,
                borrow_pool_tokens: value
                    .borrow_pool_tokens
                    .keys()
                    .filter_map(|x| x.parse::<Address>().ok())
                    .collect(),
                collateral_pool_tokens: value
                    .collateral_pool_tokens
                    .keys()
                    .filter_map(|x| x.parse::<Address>().ok())
                    .collect(),
            };

            if !position.borrow_pool_tokens.is_empty()
                && !position.collateral_pool_tokens.is_empty()
            {
                Some(position.pos_id)
            } else {
                None
            }
        })
        .collect();

    println!(
        "Found {} active positions from {} positions.",
        filtered_position_ids.len(),
        resp.data.len()
    );
    Ok(filtered_position_ids)
}

pub async fn get_init_pos_infos<
    T: Transport + ::core::clone::Clone,
    P: Provider<T, N>,
    N: Network,
>(
    // provider: &P,
    init_lens: IInitLensInstance<T, P, N>,
    pos_ids: Vec<U256>,
) -> Result<Vec<IInitLens::PosInfo>, Box<dyn std::error::Error>> {
    let pos_infos = init_lens.getInitPosInfos(pos_ids).call().await?.posInfos;
    Ok(pos_infos)
}

pub async fn get_int_pos_infos_chunk<
    T: Transport + ::core::clone::Clone,
    P: Provider<T, N> + 'static + ::core::clone::Clone,
    N: Network,
>(
    provider: P,
    pos_ids: Vec<U256>,
    chunk_size: usize,
) -> Result<Vec<IInitLens::PosInfo>, Box<dyn std::error::Error>> {
    let init_lens = IInitLens::new(INIT_LENS_ADDRESS.parse::<Address>()?, provider);
    let semaphore = Arc::new(Semaphore::new(2));
    let results = futures::stream::iter(pos_ids.chunks(chunk_size))
        .map(|chunk| {
            let semaphore = Arc::clone(&semaphore);
            let init_lens_copy = init_lens.clone();
            let chunk_vec = chunk.to_vec();

            async move {
                let _permit = semaphore.acquire().await?;
                let result = get_init_pos_infos(init_lens_copy, chunk_vec).await;
                drop(_permit);
                result
            }
        })
        .buffer_unordered(2) // Process up to 2 concurrent requests
        .collect::<Vec<Result<Vec<IInitLens::PosInfo>, _>>>()
        .await;

    // Combine all results, propagating any errors
    let flattened: Vec<IInitLens::PosInfo> =
        results.into_iter().collect::<Result<Vec<_>, _>>()?.into_iter().flatten().collect();

    Ok(flattened)
}
