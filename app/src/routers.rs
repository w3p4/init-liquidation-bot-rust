use crate::addresses::*;
use std::collections::HashMap;

pub fn get_router_map() -> HashMap<&'static str, HashMap<&'static str, &'static str>> {
    let usdc_map = HashMap::from([
        (USDC, ZERO_ADDRESS),
        (USDT, AGNI_ROUTER),
        (WETH, AGNI_ROUTER),
        (WMNT, MOE_LB_ROUTER_V2),
        (WBTC, MOE_LB_ROUTER_V2),
        (METH, AGNI_ROUTER),
        (USDE, MOE_LB_ROUTER_V2),
        (FBTC, MOE_LB_ROUTER_V2),
        (CMETH, AGNI_ROUTER),
        (USDY, MOE_LB_ROUTER_V2),
    ]);
    let usdt_map = HashMap::from([
        (USDT, ZERO_ADDRESS),
        (USDC, AGNI_ROUTER),
        (WETH, AGNI_ROUTER),
        (WMNT, AGNI_ROUTER),
        (WBTC, MOE_LB_ROUTER_V2),
        (METH, AGNI_ROUTER),
        (USDE, MOE_LB_ROUTER_V2),
        (FBTC, AGNI_ROUTER),
        (CMETH, AGNI_ROUTER),
        (USDY, AGNI_ROUTER),
    ]);
    let weth_map = HashMap::from([
        (WETH, ZERO_ADDRESS),
        (USDC, AGNI_ROUTER),
        (USDT, AGNI_ROUTER),
        (WMNT, AGNI_ROUTER),
        (WBTC, MOE_LB_ROUTER_V2),
        (METH, AGNI_ROUTER),
        (FBTC, AGNI_ROUTER),
        (USDE, MOE_LB_ROUTER_V2),
        (CMETH, AGNI_ROUTER),
        (USDY, AGNI_ROUTER),
    ]);
    let wbtc_map = HashMap::from([
        (WBTC, ZERO_ADDRESS),
        (USDC, MOE_LB_ROUTER_V2),
        (USDT, MOE_LB_ROUTER_V2),
        (WETH, MOE_LB_ROUTER_V2),
        (WMNT, MOE_LB_ROUTER_V2),
        (METH, MOE_LB_ROUTER_V2),
        (FBTC, MOE_LB_ROUTER_V2),
        (USDE, MOE_LB_ROUTER_V2),
        (CMETH, MOE_LB_ROUTER_V2),
    ]);
    let fbtc_map = HashMap::from([
        (FBTC, ZERO_ADDRESS),
        (WBTC, MOE_LB_ROUTER_V2),
        (USDC, MOE_LB_ROUTER_V2),
        (USDT, AGNI_ROUTER),
        (WETH, AGNI_ROUTER),
        (WMNT, AGNI_ROUTER),
        (METH, AGNI_ROUTER),
        (USDY, AGNI_ROUTER),
        (USDE, MOE_LB_ROUTER_V2),
        (CMETH, MOE_LB_ROUTER_V2),
    ]);

    let wmnt_map = HashMap::from([
        (WMNT, ZERO_ADDRESS),
        (USDC, AGNI_ROUTER),
        (USDT, AGNI_ROUTER),
        (WETH, AGNI_ROUTER),
        (METH, AGNI_ROUTER),
        (WBTC, MOE_LB_ROUTER_V2),
        (FBTC, AGNI_ROUTER),
        (USDE, MOE_LB_ROUTER_V2),
        (CMETH, MOE_LB_ROUTER_V2),
        (USDY, AGNI_ROUTER),
    ]);
    let meth_map = HashMap::from([
        (METH, ZERO_ADDRESS),
        (WETH, AGNI_ROUTER),
        (USDC, AGNI_ROUTER),
        (USDT, AGNI_ROUTER),
        (WMNT, AGNI_ROUTER),
        (WBTC, MOE_LB_ROUTER_V2),
        (FBTC, AGNI_ROUTER),
        (USDE, MOE_LB_ROUTER_V2),
        (CMETH, AGNI_ROUTER),
        (USDY, AGNI_ROUTER),
    ]);

    let usdy_map = HashMap::from([
        (USDY, ZERO_ADDRESS),
        (WETH, FUSION_X_ROUTER),
        (WBTC, FUSION_X_ROUTER),
        (USDT, AGNI_ROUTER),
        (USDC, AGNI_ROUTER),
        (WMNT, AGNI_ROUTER),
        (FBTC, AGNI_ROUTER),
        (USDE, MOE_LB_ROUTER_V2),
        (CMETH, AGNI_ROUTER),
        (METH, AGNI_ROUTER),
    ]);

    let usde_map = HashMap::from([
        (USDE, ZERO_ADDRESS),
        (USDT, MOE_LB_ROUTER_V2),
        (USDC, MOE_LB_ROUTER_V2),
        (WETH, MOE_LB_ROUTER_V2),
        (WBTC, MOE_LB_ROUTER_V2),
        (FBTC, MOE_LB_ROUTER_V2),
        (WMNT, MOE_LB_ROUTER_V2),
        (METH, MOE_LB_ROUTER_V2),
        (USDY, MOE_LB_ROUTER_V2),
        (CMETH, MOE_LB_ROUTER_V2),
    ]);

    let cmeth_map = HashMap::from([
        (CMETH, ZERO_ADDRESS),
        (WETH, MOE_LB_ROUTER_V2),
        (WBTC, MOE_LB_ROUTER_V2),
        (USDT, AGNI_ROUTER),
        (USDC, AGNI_ROUTER),
        (WMNT, MOE_LB_ROUTER_V2),
        (METH, AGNI_ROUTER),
        (FBTC, MOE_LB_ROUTER_V2),
        (USDE, MOE_LB_ROUTER_V2),
        (USDY, AGNI_ROUTER),
    ]);
    let router_map = HashMap::from([
        (USDC, usdc_map),
        (USDT, usdt_map),
        (WETH, weth_map),
        (WBTC, wbtc_map),
        (FBTC, fbtc_map),
        (WMNT, wmnt_map),
        (METH, meth_map),
        (USDY, usdy_map),
        (USDE, usde_map),
        (CMETH, cmeth_map),
    ]);
    router_map
}
