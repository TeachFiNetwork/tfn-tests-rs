use multiversx_sc_scenario::{rust_biguint, DebugApi};
use tfn_launchpad::common::{config::*, errors::*};

use crate::{consts::*, contracts_interactions::common::*, contracts_setup::TFNContractSetup};

#[test]
fn launchpad_new_launchpad_test() {
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
    let dao_address = sc_setup.dao_wrapper.address_ref().clone();
    let owner = sc_setup.owner.clone();
    sc_setup.blockchain_wrapper.set_egld_balance(&owner, &exp18(1));
    let start_time = 1;
    let end_time = 10;
    let mut current_time = 0;
    // new launchpad
    let launchpad_id = sc_setup.launchpad_new_launchpad(
        &dao_address,
        &owner,
        true,
        FRANCHISE1_GOVERNANCE_TOKEN_ID,
        DAO_GOVERNANCE_TOKEN_ID,
        &rust_biguint!(5).pow(18),
        &exp18(10),
        &exp18(100),
        start_time,
        end_time,
        None,
    );
    // check launchpad pending
    sc_setup.launchpad_check_launchpad_status(launchpad_id, Status::Pending, current_time);
    // advance time and check launchad active
    current_time = start_time;
    sc_setup.blockchain_wrapper.set_block_timestamp(current_time);
    sc_setup.launchpad_check_launchpad_status(launchpad_id, Status::Active, current_time);
    // advance time and check launchad ended
    current_time = end_time + 1;
    sc_setup.blockchain_wrapper.set_block_timestamp(current_time);
    sc_setup.launchpad_check_launchpad_status(launchpad_id, Status::Ended, current_time);
    // deploy and check launchpad deployed
    let franchise_address = sc_setup.launchpad_deploy_franchise(&owner, launchpad_id, None);
    sc_setup.launchpad_check_launchpad_status(launchpad_id, Status::Deployed, current_time);
    // check franchise registered in DAO
    sc_setup.dao_check_franchise_deployed(&franchise_address);
    // check franchise subscribed on Platform
    sc_setup.platform_check_is_subscribed(&franchise_address);
}

#[test]
fn launchpad_buy_launchpad_with_kyc_test() {
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
    let dao_address = sc_setup.dao_wrapper.address_ref().clone();
    let launchpad_address = sc_setup.launchpad_wrapper.address_ref().clone();
    let owner = sc_setup.owner.clone();
    sc_setup.blockchain_wrapper.set_egld_balance(&owner, &exp18(1));
    let one = exp18(1);
    let price = exp18(5);
    let tokens_to_buy = exp18(10);
    let payment_to_send = &price * &tokens_to_buy / &one;
    let amount_to_sell = exp18(100);
    sc_setup.blockchain_wrapper.set_esdt_balance(&owner, DAO_GOVERNANCE_TOKEN_ID.as_bytes(), &payment_to_send);
    sc_setup.blockchain_wrapper.set_esdt_balance(&owner, FRANCHISE1_GOVERNANCE_TOKEN_ID.as_bytes(), &amount_to_sell);
    let start_time = 1;
    let end_time = 10;
    // new launchpad
    let launchpad_id = sc_setup.launchpad_new_launchpad(
        &dao_address,
        &owner,
        true,
        FRANCHISE1_GOVERNANCE_TOKEN_ID,
        DAO_GOVERNANCE_TOKEN_ID,
        &price,
        &one,
        &exp18(100),
        start_time,
        end_time,
        None,
    );
    // add tokens
    sc_setup.launchpad_add_tokens(&owner, launchpad_id, FRANCHISE1_GOVERNANCE_TOKEN_ID, &amount_to_sell, None);
    // advance time so we can buy
    sc_setup.blockchain_wrapper.set_block_timestamp(start_time);
    // buy should fail since we are not whitelisted
    sc_setup.launchpad_buy(&owner, launchpad_id, DAO_GOVERNANCE_TOKEN_ID, &payment_to_send, Some(ERROR_NOT_WHITELISTED));
    // whitelist user
    sc_setup.launchpad_whitelist_user(&owner, launchpad_id, &owner, None);
    // buy
    sc_setup.launchpad_buy(&owner, launchpad_id, DAO_GOVERNANCE_TOKEN_ID, &payment_to_send, None);
    // cancel launchpad should fail since someone already participated
    sc_setup.launchpad_cancel_launchpad(&owner, launchpad_id, Some(ERROR_DELETING_LAUNCHPAD));
    // check balances
    sc_setup.blockchain_wrapper.check_esdt_balance(&owner, FRANCHISE1_GOVERNANCE_TOKEN_ID.as_bytes(), &tokens_to_buy);
    sc_setup.blockchain_wrapper.check_esdt_balance(&launchpad_address, DAO_GOVERNANCE_TOKEN_ID.as_bytes(), &payment_to_send);
    sc_setup.blockchain_wrapper.check_esdt_balance(&launchpad_address, FRANCHISE1_GOVERNANCE_TOKEN_ID.as_bytes(), &(&amount_to_sell - &tokens_to_buy));
    // advance time to end launchpad
    sc_setup.blockchain_wrapper.set_block_timestamp(end_time + 1);
    // deploy franchise
    let franchise_address = sc_setup.launchpad_deploy_franchise(&owner, launchpad_id, None);
    // check franchise balances
    sc_setup.blockchain_wrapper.check_esdt_balance(&franchise_address, DAO_GOVERNANCE_TOKEN_ID.as_bytes(), &payment_to_send);
    sc_setup.blockchain_wrapper.check_esdt_balance(&franchise_address, FRANCHISE1_GOVERNANCE_TOKEN_ID.as_bytes(), &(amount_to_sell - tokens_to_buy));
}

#[test]
fn launchpad_cancel_test() {
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
    let dao_address = sc_setup.dao_wrapper.address_ref().clone();
    let owner = sc_setup.owner.clone();
    // new launchpad
    let launchpad_id = sc_setup.launchpad_new_launchpad(
        &dao_address,
        &owner,
        true,
        FRANCHISE1_GOVERNANCE_TOKEN_ID,
        DAO_GOVERNANCE_TOKEN_ID,
        &rust_biguint!(5).pow(18),
        &exp18(10),
        &exp18(100),
        1,
        2,
        None,
    );
    // cancel launchpad
    sc_setup.launchpad_cancel_launchpad(&owner, launchpad_id, None);
    // check launchpad removed
    sc_setup.launchpad_check_last_launchpad_id(1);
}
