use foundry_contracts::ierc20liquidationbot::IERC20LiquidationBot::{
    self, getLiquidationInfoReturn,
};

use alloy::{
    contract::private::{Network, Provider, Transport},
    primitives::utils::parse_ether,
    primitives::{Address, U256},
};

use crate::{addresses::WMNT, routers::get_router_map, tokens::token_pool_mapping};

const LIQUIDATION_BOT: &str = "0x47dcA5d272b46A578F94e343983d0C9A43C6AdEf";

pub async fn get_best_liquidation<
    T: Transport + ::core::clone::Clone,
    P: Provider<T, N> + 'static + ::core::clone::Clone,
    N: Network,
>(
    provider: P,
    pos_id: &U256,
) -> Result<getLiquidationInfoReturn, Box<dyn std::error::Error>> {
    let liq_bot = IERC20LiquidationBot::new(LIQUIDATION_BOT.parse::<Address>()?, provider.clone());
    let liq_info = liq_bot.getLiquidationInfo(*pos_id).call().await?;
    Ok(liq_info)
}

pub async fn try_liquidate<
    T: Transport + ::core::clone::Clone,
    P: Provider<T, N> + 'static + ::core::clone::Clone,
    N: Network,
>(
    provider: P,
    pos_id: &U256,
    profit: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // get best pool
    let liq_info = get_best_liquidation(provider.clone(), pos_id).await?;
    let repay_pool = liq_info.bestPoolToRepay.to_string();
    let coll_pool = liq_info.bestPoolOut.to_string();

    // get tokens from pools
    let pool_token_map = token_pool_mapping().pool_token;
    let repay_token = pool_token_map.get(repay_pool.as_str()).ok_or("Repay token not found")?;
    let token_out = pool_token_map.get(coll_pool.as_str()).ok_or("Collateral token not found")?;

    let liq_bot = IERC20LiquidationBot::new(LIQUIDATION_BOT.parse::<Address>()?, provider.clone());

    let borrow_token = repay_token;

    let router_map = get_router_map();
    let router_1 = router_map
        .get(borrow_token)
        .ok_or("Router 1 not found")?
        .get(repay_token)
        .ok_or("Router 1 token not found")?;
    let router_2 = router_map
        .get(token_out)
        .ok_or("Router 2 not found")?
        .get(borrow_token)
        .ok_or("Router 2 token not found")?;
    let router_3 = router_map
        .get(token_out)
        .ok_or("Router 2 not found")?
        .get(WMNT)
        .ok_or("Router 2 token not found")?;

    let _ = liq_bot
        .flashLiquidateReturnNative(
            *pos_id,
            repay_pool.parse::<Address>()?,
            router_1.parse::<Address>()?,
            repay_pool.parse::<Address>()?,
            router_2.parse::<Address>()?,
            coll_pool.parse::<Address>()?,
            router_3.parse::<Address>()?,
            parse_ether(profit).unwrap(),
        )
        .send()
        .await?;
    Ok(())
}
