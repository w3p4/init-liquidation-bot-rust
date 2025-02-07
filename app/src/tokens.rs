use crate::addresses::*;
use std::collections::HashMap;

pub struct TokenPoolMapping {
    pub token_pool: HashMap<&'static str, &'static str>,
    pub pool_token: HashMap<&'static str, &'static str>,
}

pub fn token_pool_mapping() -> TokenPoolMapping {
    let token_pool = HashMap::from([
        (WMNT, POOL_WMNT),
        (USDC, POOL_USDC),
        (USDT, POOL_USDT),
        (WETH, POOL_WETH),
        (WBTC, POOL_WBTC),
        (USDY, POOL_USDY),
        (USDE, POOL_USDE),
        (METH, POOL_METH),
        (FBTC, POOL_FBTC),
        (CMETH, POOL_CMETH),
    ]);
    let pool_token = token_pool.clone().into_iter().map(|(k, v)| (v, k)).collect();
    TokenPoolMapping { token_pool, pool_token }
}
