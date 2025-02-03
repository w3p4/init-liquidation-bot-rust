use phf::{phf_map, Map};

use crate::addresses::{
    AGNI_ROUTER, CMETH, FBTC, FUSION_X_ROUTER, METH, MOE_LB_ROUTER_V2, USDC, USDE, USDT, USDY,
    WETH, ZERO,
};

static USDC_PAIRS: Map<&'static str, &'static str> = phf_map! {
    USDC => ZERO,
    USDT => AGNI_ROUTER,
    WETH => AGNI_ROUTER,
    WMNT => MOE_LB_ROUTER_V2,
    WBTC => MOE_LB_ROUTER_V2,
    METH => AGNI_ROUTER
    USDE => MOE_LB_ROUTER_V2,
    FBTC => MOE_LB_ROUTER_V2,
    CMETH => AGNI_ROUTER,
    USDY => MOE_LB_ROUTER_V2,
};

static USDT_PAIRS: Map<&'static str, &'static str> = phf_map! {
    USDT => ZERO,
    USDC => AGNI_ROUTER,
    WETH => AGNI_ROUTER,
    WMNT => AGNI_ROUTER,
    WBTC => MOE_LB_ROUTER_V2,
    METH => AGNI_ROUTER,
    USDE => MOE_LB_ROUTER_V2,
    FBTC => AGNI_ROUTER,
    CMETH => AGNI_ROUTER,
    USDY => AGNI_ROUTER,
};

static WETH_PAIRS: Map<&'static str, &'static str> = phf_map! {
    WETH => ZERO,
    USDC => AGNI_ROUTER,
    USDT => AGNI_ROUTER,
    WMNT => AGNI_ROUTER,
    WBTC => MOE_LB_ROUTER_V2,
    METH => AGNI_ROUTER,
    USDE => MOE_LB_ROUTER_V2,
    FBTC => AGNI_ROUTER,
    CMETH => AGNI_ROUTER,
    USDY => AGNI_ROUTER,
};

static WBTC_PAIRS: Map<&'static str, &'static str> = phf_map! {
    WBTC => ZERO,
    USDC => MOE_LB_ROUTER_V2,
    USDT => MOE_LB_ROUTER_V2,
    WETH => MOE_LB_ROUTER_V2,
    WMNT => MOE_LB_ROUTER_V2,
    METH => MOE_LB_ROUTER_V2,
    FBTC => MOE_LB_ROUTER_V2,
    USDE => MOE_LB_ROUTER_V2,
    CMETH => MOE_LB_ROUTER_V2,
};

static FBTC_PAIRS: Map<&'static str, &'static str> = phf_map! {
    FBTC => ZERO,
    WBTC => MOE_LB_ROUTER_V2,
    USDC => MOE_LB_ROUTER_V2,
    USDT => AGNI_ROUTER,
    WETH => AGNI_ROUTER,
    WMNT => AGNI_ROUTER,
    METH => AGNI_ROUTER,
    USDY => AGNI_ROUTER,
    USDE => MOE_LB_ROUTER_V2,
    CMETH => MOE_LB_ROUTER_V2,
};

static WMNT_PAIRS: Map<&'static str, &'static str> = phf_map! {
    WMNT => ZERO,
    USDC => AGNI_ROUTER,
    USDT => AGNI_ROUTER,
    WETH => AGNI_ROUTER,
    METH => AGNI_ROUTER,
    WBTC => MOE_LB_ROUTER_V2,
    FBTC => AGNI_ROUTER,
    USDE => MOE_LB_ROUTER_V2,
    CMETH => AGNI_ROUTER,
    USDY => AGNI_ROUTER,
};

static METH_PAIRS: Map<&'static str, &'static str> = phf_map! {
    METH => ZERO,
    WETH => AGNI_ROUTER,
    USDC => AGNI_ROUTER,
    USDT => AGNI_ROUTER,
    WMNT => AGNI_ROUTER,
    WBTC => MOE_LB_ROUTER_V2,
    FBTC => AGNI_ROUTER,
    USDE => MOE_LB_ROUTER_V2,
    CMETH => AGNI_ROUTER,
    USDY => AGNI_ROUTER,
};

static USDY_PAIRS: Map<&'static str, &'static str> = phf_map! {
    USDY => ZERO,
    WETH => FUSION_X_ROUTER,
    WBTC => FUSION_X_ROUTER,
    USDT => AGNI_ROUTER,
    USDC => AGNI_ROUTER,
    WMNT => AGNI_ROUTER,
    FBTC => AGNI_ROUTER,
    USDE => MOE_LB_ROUTER_V2,
    CMETH => AGNI_ROUTER,
    METH => AGNI_ROUTER,
};

static USDE_PAIRS: Map<&'static str, &'static str> = phf_map! {
    USDE => ZERO,
    USDT => MOE_LB_ROUTER_V2,
    USDC => MOE_LB_ROUTER_V2,
    WETH => MOE_LB_ROUTER_V2,
    WBTC => MOE_LB_ROUTER_V2,
    FBTC => MOE_LB_ROUTER_V2,
    WMNT => MOE_LB_ROUTER_V2,
    METH => MOE_LB_ROUTER_V2,
    USDY => MOE_LB_ROUTER_V2,
    CMETH => MOE_LB_ROUTER_V2,
};

static CMETH_PAIRS: Map<&'static str, &'static str> = phf_map! {
     CMETH => ZERO,
     WETH => MOE_LB_ROUTER_V2,
     WBTC => MOE_LB_ROUTER_V2,
     USDT => AGNI_ROUTER,
     USDC => AGNI_ROUTER,
     WMNT => MOE_LB_ROUTER_V2,
     METH => AGNI_ROUTER,
     FBTC => MOE_LB_ROUTER_V2,
     USDE => MOE_LB_ROUTER_V2,
     USDY => AGNI_ROUTER,
};

/// The best router mapping for each token pair
pub static BEST_ROUTER: Map<&'static str, &'static Map<&'static str, &'static str>> = phf_map! {
    USDC => &USDC_PAIRS,
    USDT => &USDT_PAIRS,
    WETH => &WETH_PAIRS,
    WBTC => &WBTC_PAIRS,
    FBTC => &FBTC_PAIRS,
    WMNT => &WMNT_PAIRS,
    METH => &METH_PAIRS,
    USDY => &USDY_PAIRS,
    USDE => &USDE_PAIRS,
    CMETH => &CMETH_PAIRS,
};
