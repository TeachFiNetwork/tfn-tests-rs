use multiversx_sc_scenario::DebugApi;
use tfn_dex::common::errors::*;

use crate::{consts::*, contracts_setup::TFNContractSetup};

#[test]
fn test_dex_create_pair_test() {
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
    // whitelist owner
    sc_setup.platform_whitelist_address(&owner, &owner, None);
    // create pair - should fail since FRANCHISE1_GOVERNANCE_TOKEN_ID is not registered as base token
    sc_setup.test_dex_create_pair( 
        &owner,
        DAO_GOVERNANCE_TOKEN_ID,
        FRANCHISE1_GOVERNANCE_TOKEN_ID,
        18,
        LP_FEE,
        OWNER_FEE,
        Some(ERROR_WRONG_BASE_TOKEN),
    );
    // create pair
    sc_setup.test_dex_create_pair( 
        &owner,
        FRANCHISE1_GOVERNANCE_TOKEN_ID,
        DAO_GOVERNANCE_TOKEN_ID,
        18,
        LP_FEE,
        OWNER_FEE,
        None,
    );
    // check pair created
    sc_setup.test_dex_check_pairs_count(1);
    sc_setup.test_dex_pair_exists_by_tickers(FRANCHISE1_GOVERNANCE_TOKEN_ID, DAO_GOVERNANCE_TOKEN_ID);
}
