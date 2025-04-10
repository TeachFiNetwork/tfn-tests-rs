use multiversx_sc_scenario::{rust_biguint, DebugApi};

use tfn_dex::common::{consts::MAX_PERCENT, errors::*};
use crate::{consts::*, contracts_interactions::common::*, contracts_setup::TFNContractSetup};

#[test]
fn dex_create_pair_test() {
    DebugApi::dummy();
    let mut sc_setup = TFNContractSetup::new(
        tfn_dao::contract_obj,
        tfn_dex::contract_obj,
        tfn_platform::contract_obj,
        tfn_franchise_dao::contract_obj,
        tfn_launchpad::contract_obj,
        tfn_staking::contract_obj,
        tfn_test_launchpad::contract_obj,
        tfn_test_staking::contract_obj,
        tfn_test_dex::contract_obj,
        tfn_nft_marketplace::contract_obj,
        tfn_digital_identity::contract_obj,
    );
    let owner = sc_setup.owner.clone();
    // create pair - should fail since FRANCHISE1_GOVERNANCE_TOKEN_ID is not registered as base token
    sc_setup.dex_create_pair( &owner, DAO_GOVERNANCE_TOKEN_ID, FRANCHISE1_GOVERNANCE_TOKEN_ID, Some(ERROR_WRONG_BASE_TOKEN));
    // create pair
    sc_setup.dex_create_pair( &owner, FRANCHISE1_GOVERNANCE_TOKEN_ID, DAO_GOVERNANCE_TOKEN_ID, None);
    // check pair created
    sc_setup.dex_check_pairs_count(1);
    sc_setup.dex_pair_exists_by_tickers(FRANCHISE1_GOVERNANCE_TOKEN_ID, DAO_GOVERNANCE_TOKEN_ID);
}

#[test]
fn dex_liquidity_test() {
    DebugApi::dummy();
    let mut sc_setup = TFNContractSetup::new(
        tfn_dao::contract_obj,
        tfn_dex::contract_obj,
        tfn_platform::contract_obj,
        tfn_franchise_dao::contract_obj,
        tfn_launchpad::contract_obj,
        tfn_staking::contract_obj,
        tfn_test_launchpad::contract_obj,
        tfn_test_staking::contract_obj,
        tfn_test_dex::contract_obj,
        tfn_nft_marketplace::contract_obj,
        tfn_digital_identity::contract_obj,
    );
    let owner = sc_setup.owner.clone();
    let user = sc_setup.setup_new_user(1u64);
    let token_amount = exp18(100);
    let base_token_amount = exp18(1000);
    sc_setup.blockchain_wrapper.set_esdt_balance(&owner, DAO_GOVERNANCE_TOKEN_ID.as_bytes(), &base_token_amount);
    sc_setup.blockchain_wrapper.set_esdt_balance(&owner, FRANCHISE1_GOVERNANCE_TOKEN_ID.as_bytes(), &token_amount);
    sc_setup.blockchain_wrapper.set_esdt_balance(&user, DAO_GOVERNANCE_TOKEN_ID.as_bytes(), &base_token_amount);
    sc_setup.blockchain_wrapper.set_esdt_balance(&user, FRANCHISE1_GOVERNANCE_TOKEN_ID.as_bytes(), &token_amount);
    // create pair
    sc_setup.dex_create_pair( &owner, FRANCHISE1_GOVERNANCE_TOKEN_ID, DAO_GOVERNANCE_TOKEN_ID, None);
    // add initial liquidity should fail - only owner can add initial liquidity and set the price
    sc_setup.dex_add_liquidity(
        &user,
        FRANCHISE1_GOVERNANCE_TOKEN_ID,
        &token_amount,
        DAO_GOVERNANCE_TOKEN_ID,
        &base_token_amount,
        Some(ERROR_ONLY_OWNER_OR_LAUNCHPAD),
    );
    // add initial liquidity
    sc_setup.dex_add_liquidity(
        &owner,
        FRANCHISE1_GOVERNANCE_TOKEN_ID,
        &token_amount,
        DAO_GOVERNANCE_TOKEN_ID,
        &base_token_amount,
        None,
    );
    // check lp received
    let lp_token = sc_setup.dex_get_pair_lp_token_by_tickers(FRANCHISE1_GOVERNANCE_TOKEN_ID, DAO_GOVERNANCE_TOKEN_ID);
    sc_setup.blockchain_wrapper.check_esdt_balance(&owner, lp_token.as_slice(), &base_token_amount);
    // remove liquidity
    sc_setup.dex_remove_liquidity(
        &owner,
        err2str(lp_token.as_slice()),
        &base_token_amount,
        None,
    );
    // check balances
    sc_setup.blockchain_wrapper.check_esdt_balance(&owner, DAO_GOVERNANCE_TOKEN_ID.as_bytes(), &base_token_amount);
    sc_setup.blockchain_wrapper.check_esdt_balance(&owner, FRANCHISE1_GOVERNANCE_TOKEN_ID.as_bytes(), &token_amount);
}

#[test]
fn dex_swap_fixed_input_test() {
    DebugApi::dummy();
    let mut sc_setup = TFNContractSetup::new(
        tfn_dao::contract_obj,
        tfn_dex::contract_obj,
        tfn_platform::contract_obj,
        tfn_franchise_dao::contract_obj,
        tfn_launchpad::contract_obj,
        tfn_staking::contract_obj,
        tfn_test_launchpad::contract_obj,
        tfn_test_staking::contract_obj,
        tfn_test_dex::contract_obj,
        tfn_nft_marketplace::contract_obj,
        tfn_digital_identity::contract_obj,
    );
    let owner = sc_setup.owner.clone();
    let swap_base_amount = exp18(10);
    let token_amount = exp18(100);
    let base_token_amount = exp18(1000);
    sc_setup.blockchain_wrapper.set_esdt_balance(&owner, DAO_GOVERNANCE_TOKEN_ID.as_bytes(), &(&base_token_amount + &swap_base_amount));
    sc_setup.blockchain_wrapper.set_esdt_balance(&owner, FRANCHISE1_GOVERNANCE_TOKEN_ID.as_bytes(), &token_amount);
    // create pair
    sc_setup.dex_create_pair( &owner, FRANCHISE1_GOVERNANCE_TOKEN_ID, DAO_GOVERNANCE_TOKEN_ID, None);
    // add initial liquidity
    sc_setup.dex_add_liquidity(
        &owner,
        FRANCHISE1_GOVERNANCE_TOKEN_ID,
        &token_amount,
        DAO_GOVERNANCE_TOKEN_ID,
        &base_token_amount,
        None,
    );
    // set pair active
    let pair_id = sc_setup.dex_get_pair_id_by_tickers(FRANCHISE1_GOVERNANCE_TOKEN_ID, DAO_GOVERNANCE_TOKEN_ID);
    sc_setup.dex_set_pair_active(&owner, pair_id.unwrap(), None);
    // set owner fee
    sc_setup.dex_set_owner_fee(&owner, OWNER_FEE, None);
    // swap fixed input
    let amount_out = sc_setup.dex_get_amount_out(DAO_GOVERNANCE_TOKEN_ID, FRANCHISE1_GOVERNANCE_TOKEN_ID, &swap_base_amount);
    sc_setup.dex_swap_fixed_input(
        &owner,
        DAO_GOVERNANCE_TOKEN_ID,
        &swap_base_amount,
        FRANCHISE1_GOVERNANCE_TOKEN_ID,
        &(&swap_base_amount / rust_biguint!(20)),
        None,
    );
    // check balances
    sc_setup.blockchain_wrapper.check_esdt_balance(&owner, FRANCHISE1_GOVERNANCE_TOKEN_ID.as_bytes(), &amount_out);
    // check cummulated fees
    let expected_fees: Vec<(Vec<u8>, num_bigint::BigUint)> = vec![(DAO_GOVERNANCE_TOKEN_ID.as_bytes().to_vec(), swap_base_amount * OWNER_FEE / MAX_PERCENT)];
    sc_setup.dex_check_cummulated_fees(expected_fees);
}

#[test]
fn dex_swap_fixed_output_test() {
    DebugApi::dummy();
    let mut sc_setup = TFNContractSetup::new(
        tfn_dao::contract_obj,
        tfn_dex::contract_obj,
        tfn_platform::contract_obj,
        tfn_franchise_dao::contract_obj,
        tfn_launchpad::contract_obj,
        tfn_staking::contract_obj,
        tfn_test_launchpad::contract_obj,
        tfn_test_staking::contract_obj,
        tfn_test_dex::contract_obj,
        tfn_nft_marketplace::contract_obj,
        tfn_digital_identity::contract_obj,
    );
    let owner = sc_setup.owner.clone();
    let token_amount_wanted = exp18(1);
    let token_amount = exp18(100);
    let base_token_amount = exp18(1000);
    sc_setup.blockchain_wrapper.set_esdt_balance(&owner, DAO_GOVERNANCE_TOKEN_ID.as_bytes(), &base_token_amount);
    sc_setup.blockchain_wrapper.set_esdt_balance(&owner, FRANCHISE1_GOVERNANCE_TOKEN_ID.as_bytes(), &token_amount);
    // create pair
    sc_setup.dex_create_pair( &owner, FRANCHISE1_GOVERNANCE_TOKEN_ID, DAO_GOVERNANCE_TOKEN_ID, None);
    // add initial liquidity
    sc_setup.dex_add_liquidity(
        &owner,
        FRANCHISE1_GOVERNANCE_TOKEN_ID,
        &token_amount,
        DAO_GOVERNANCE_TOKEN_ID,
        &base_token_amount,
        None,
    );
    // set pair active
    let pair_id = sc_setup.dex_get_pair_id_by_tickers(FRANCHISE1_GOVERNANCE_TOKEN_ID, DAO_GOVERNANCE_TOKEN_ID);
    sc_setup.dex_set_pair_active(&owner, pair_id.unwrap(), None);
    // set owner fee
    sc_setup.dex_set_owner_fee(&owner, OWNER_FEE, None);
    // calculate amount in and set balance
    let amount_in = sc_setup.dex_get_amount_in(DAO_GOVERNANCE_TOKEN_ID, FRANCHISE1_GOVERNANCE_TOKEN_ID, &token_amount_wanted);
    sc_setup.blockchain_wrapper.set_esdt_balance(&owner, DAO_GOVERNANCE_TOKEN_ID.as_bytes(), &amount_in);
    // swap fixed output
    sc_setup.dex_swap_fixed_output(
        &owner,
        DAO_GOVERNANCE_TOKEN_ID,
        &amount_in,
        FRANCHISE1_GOVERNANCE_TOKEN_ID,
        &token_amount_wanted,
        None,
    );
    // check balances
    sc_setup.blockchain_wrapper.check_esdt_balance(&owner, FRANCHISE1_GOVERNANCE_TOKEN_ID.as_bytes(), &token_amount_wanted);
    // check cummulated fees
    let expected_fees: Vec<(Vec<u8>, num_bigint::BigUint)> = vec![(DAO_GOVERNANCE_TOKEN_ID.as_bytes().to_vec(), amount_in * OWNER_FEE / MAX_PERCENT)];
    sc_setup.dex_check_cummulated_fees(expected_fees);
}
