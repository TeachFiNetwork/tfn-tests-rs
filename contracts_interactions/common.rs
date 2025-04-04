use multiversx_sc::types::{Address, EsdtLocalRole};
use multiversx_sc_scenario::{imports::TxResult, DebugApi, rust_biguint, num_bigint};
use tfn_dex::common::consts::*;

use crate::contracts_setup::TFNContractSetup;

use std::ops::Mul;

impl<
    TFNDAOContractObjBuilder,
    TFNDEXContractObjBuilder,
    TFNPlatformContractObjBuilder,
    TFNFranchiseDAOContractObjBuilder,
    TFNLaunchpadContractObjBuilder,
    TFNStakingContractObjBuilder,
    TFNTestLaunchpadContractObjBuilder,
    TFNTestStakingContractObjBuilder,
    TFNTestDEXContractObjBuilder,
    TFNNFTMarketplaceContractObjBuilder,
    TFNDigitalIdentityContractObjBuilder
>
TFNContractSetup<
    TFNDAOContractObjBuilder,
    TFNDEXContractObjBuilder,
    TFNPlatformContractObjBuilder,
    TFNFranchiseDAOContractObjBuilder,
    TFNLaunchpadContractObjBuilder,
    TFNStakingContractObjBuilder,
    TFNTestLaunchpadContractObjBuilder,
    TFNTestStakingContractObjBuilder,
    TFNTestDEXContractObjBuilder,
    TFNNFTMarketplaceContractObjBuilder,
    TFNDigitalIdentityContractObjBuilder
>
where
    TFNDAOContractObjBuilder: 'static + Copy + Fn() -> tfn_dao::ContractObj<DebugApi>,
    TFNDEXContractObjBuilder: 'static + Copy + Fn() -> tfn_dex::ContractObj<DebugApi>,
    TFNPlatformContractObjBuilder: 'static + Copy + Fn() -> tfn_platform::ContractObj<DebugApi>,
    TFNFranchiseDAOContractObjBuilder: 'static + Copy + Fn() -> tfn_franchise_dao::ContractObj<DebugApi>,
    TFNLaunchpadContractObjBuilder: 'static + Copy + Fn() -> tfn_launchpad::ContractObj<DebugApi>,
    TFNStakingContractObjBuilder: 'static + Copy + Fn() -> tfn_staking::ContractObj<DebugApi>,
    TFNTestLaunchpadContractObjBuilder: 'static + Copy + Fn() -> tfn_test_launchpad::ContractObj<DebugApi>,
    TFNTestStakingContractObjBuilder: 'static + Copy + Fn() -> tfn_test_staking::ContractObj<DebugApi>,
    TFNTestDEXContractObjBuilder: 'static + Copy + Fn() -> tfn_test_dex::ContractObj<DebugApi>,
    TFNNFTMarketplaceContractObjBuilder: 'static + Copy + Fn() -> tfn_nft_marketplace::ContractObj<DebugApi>,
    TFNDigitalIdentityContractObjBuilder: 'static + Copy + Fn() -> tfn_digital_identity::ContractObj<DebugApi>,
{
    pub fn setup_new_user(
        &mut self,
        egld_amount: u64
    ) -> Address {
        self.blockchain_wrapper.create_user_account(&exp18(egld_amount))
    }

    pub fn handle_error(&mut self, result: &TxResult, err: Option<&[u8]>) {
        match err {
            Some(err) => {
                result.assert_user_error(err2str(err));
            }
            None => {
                result.assert_ok();
            }
        }
    }
}

pub static DEFAULT_ROLES: &[EsdtLocalRole] = &[
    EsdtLocalRole::Mint,
    EsdtLocalRole::Burn,
    EsdtLocalRole::Transfer,
];

pub fn err2str(err: &[u8]) -> &str {
    std::str::from_utf8(err).unwrap()
}

pub fn exp18(value: u64) -> num_bigint::BigUint {
    value.mul(rust_biguint!(10).pow(18))
}

pub fn get_lp_token_id(token: &str, base_token: &str) -> (Vec<u8>, Vec<u8>) {
    let token_ticker = token.split("-").next().unwrap();
    let base_token_ticker = base_token.split("-").next().unwrap();
    let mut lp_ticker = [token_ticker, base_token_ticker].join("");
    let prefix_suffix_len = LP_TOKEN_PREFIX.len() + LP_TOKEN_SUFFIX.len();
    let max_ticker_len = 20 - prefix_suffix_len; 
    if lp_ticker.len() > max_ticker_len {
        lp_ticker = lp_ticker[..max_ticker_len].to_string();
    }
    let lp_name = [
        err2str(LP_TOKEN_PREFIX),
        err2str(lp_ticker.as_bytes()),
        err2str(LP_TOKEN_SUFFIX)
    ].join("");
    if lp_ticker.len() > 10 {
        lp_ticker = lp_ticker[..10].to_string();
    }
    lp_ticker = [err2str(lp_ticker.as_bytes()), "123456"].join("-").to_string();

    (lp_name.as_bytes().to_vec(), lp_ticker.as_bytes().to_vec())
}
