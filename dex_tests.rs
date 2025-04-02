use multiversx_sc_scenario::DebugApi;

use tfn_dex::common::errors::*;
use crate::{consts::*, contracts_interactions::common::exp18, contracts_setup::TFNContractSetup};

#[test]
fn dex_create_pair_test() {
    DebugApi::dummy();
    let mut sc_setup = TFNContractSetup::new(
        tfn_dao::contract_obj,
        tfn_dex::contract_obj,
        tfn_platform::contract_obj,
        tfn_franchise_dao::contract_obj,
        tfn_employee::contract_obj,
        tfn_student::contract_obj,
        tfn_launchpad::contract_obj,
        tfn_staking::contract_obj,
        tfn_test_launchpad::contract_obj,
        tfn_test_staking::contract_obj,
        tfn_test_dex::contract_obj,
        tfn_nft_marketplace::contract_obj,
    );
    let owner = sc_setup.owner.clone();
    // create pair - should fail since FRANCHISE1_GOVERNANCE_TOKEN_ID is not registered as base token
    sc_setup.dex_create_pair( &owner, DAO_GOVERNANCE_TOKEN_ID, FRANCHISE1_GOVERNANCE_TOKEN_ID, 18, Some(ERROR_WRONG_BASE_TOKEN));
    // create pair
    sc_setup.dex_create_pair( &owner, FRANCHISE1_GOVERNANCE_TOKEN_ID, DAO_GOVERNANCE_TOKEN_ID, 18, None);
    // check pair created
    sc_setup.dex_check_pairs_count(1);
    sc_setup.dex_pair_exists_by_tickers(FRANCHISE1_GOVERNANCE_TOKEN_ID, DAO_GOVERNANCE_TOKEN_ID);
}

#[test]
fn dex_add_liquidity_test() {
    DebugApi::dummy();
    let mut sc_setup = TFNContractSetup::new(
        tfn_dao::contract_obj,
        tfn_dex::contract_obj,
        tfn_platform::contract_obj,
        tfn_franchise_dao::contract_obj,
        tfn_employee::contract_obj,
        tfn_student::contract_obj,
        tfn_launchpad::contract_obj,
        tfn_staking::contract_obj,
        tfn_test_launchpad::contract_obj,
        tfn_test_staking::contract_obj,
        tfn_test_dex::contract_obj,
        tfn_nft_marketplace::contract_obj,
    );
    let owner = sc_setup.owner.clone();
    let token_amount = exp18(100);
    let base_token_amount = exp18(1000);
    sc_setup.blockchain_wrapper.set_esdt_balance(&owner, DAO_GOVERNANCE_TOKEN_ID.as_bytes(), &base_token_amount);
    sc_setup.blockchain_wrapper.set_esdt_balance(&owner, FRANCHISE1_GOVERNANCE_TOKEN_ID.as_bytes(), &token_amount);
    // create pair - should fail since FRANCHISE1_GOVERNANCE_TOKEN_ID is not registered as base token
    sc_setup.dex_create_pair( &owner, DAO_GOVERNANCE_TOKEN_ID, FRANCHISE1_GOVERNANCE_TOKEN_ID, 18, Some(ERROR_WRONG_BASE_TOKEN));
    // create pair
    sc_setup.dex_create_pair( &owner, FRANCHISE1_GOVERNANCE_TOKEN_ID, DAO_GOVERNANCE_TOKEN_ID, 18, None);
    // add liquidity
    sc_setup.dex_add_liquidity(
        &owner,
        FRANCHISE1_GOVERNANCE_TOKEN_ID,
        token_amount,
        DAO_GOVERNANCE_TOKEN_ID,
        base_token_amount.clone(),
        None,
    );
    // check lp received
    let lp_token = sc_setup.dex_get_pair_lp_token_by_tickers(FRANCHISE1_GOVERNANCE_TOKEN_ID, DAO_GOVERNANCE_TOKEN_ID);
    sc_setup.blockchain_wrapper.check_esdt_balance(&owner, lp_token.as_slice(), &base_token_amount);
}
