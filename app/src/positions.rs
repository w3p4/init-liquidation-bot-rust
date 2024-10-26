use std::sync::Arc;
use futures::TryFutureExt;
// use tokio::sync
// use futures::future::join_all;
// use futures::join;
use serde::Deserialize;
use tokio::sync::Semaphore;
use std::collections::HashMap;

use alloy::{
    contract::private::{Network, Provider, Transport},
    primitives::{utils::format_units, Address, U256},
};

use foundry_contracts::iinitlens::IInitLens;

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

pub async fn get_init_pos_infos<
    T: Transport + ::core::clone::Clone,
    P: Provider<T, N>,
    N: Network,
>(
    provider: &P,
    pos_ids: Vec<U256>,
) -> Result<Vec<IInitLens::PosInfo>, Box<dyn std::error::Error>> {
    // init lens instance
    let init_lens = IInitLens::new(INIT_LENS_ADDRESS.parse::<Address>()?, provider);

    let builder = init_lens.getInitPosInfos(pos_ids);

    // call
    let data = builder.call().await?;

    // destruct return data
    let pos_infos = data.posInfos;
    Ok(pos_infos)
}

pub async fn get_int_pos_infos_chunk<
    T: Transport + ::core::clone::Clone,
    P: Provider<T, N> + 'static  + ::core::clone::Clone,
    N: Network,
>(
    provider: P,
    pos_ids: Vec<U256>,
    chunks: usize,
) -> Result<Vec<IInitLens::PosInfo>, Box<dyn std::error::Error>> {
    let semaphore = Arc::new(Semaphore::new(10));
    let chunks = pos_ids.chunks(chunks);

    let mut handles = Vec::new();

    for chunk in chunks {
        let semaphore = semaphore.clone();
        let provider = provider.clone();
        let chunk_vec = chunk.to_vec();
        let jh = tokio::spawn(async move{
            let _permit = semaphore.acquire().await.unwrap();
            let response = get_init_pos_infos(&provider, chunk_vec).await.unwrap();
            drop(_permit);
            response
        });
        handles.push(jh);
    }

    let mut responses = Vec::new();
    for handle in handles{
        let response = handle.await.unwrap();
        responses.push(response);
    }
   let flattened_responses = responses.into_iter().flatten().collect::<Vec<IInitLens::PosInfo>>();
    Ok(flattened_responses)
}































































































