use multiversx_sc_scenario::{managed_address, managed_token_id, rust_biguint, DebugApi};
use tfn_dao::{common::errors::*, multisig::MultisigModule, common::config::*};

use crate::{consts::*, contracts_setup::TFNContractSetup, *};

#[test]
fn dao_add_new_board_member_test() {
    DebugApi::dummy();
    let big_zero = rust_biguint!(0u64);
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
    let new_board_member = sc_setup.setup_new_user(1u64);
    let mut action_id: usize = 0;
    // new member propose add new board member - should fail
    sc_setup.blockchain_wrapper
        .execute_tx(&new_board_member, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.propose_add_board_member(managed_address!(&new_board_member));
        })
        .assert_user_error(err2str(ERROR_ONLY_BOARD_MEMBERS));
    // owner propose add new board member
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            action_id = sc.propose_add_board_member(managed_address!(&new_board_member));
        })
        .assert_ok();
    // perform action - add new board member
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.perform_action(action_id);
        })
        .assert_ok();
    // propose add new board member again - should fail
    sc_setup.blockchain_wrapper
        .execute_tx(&new_board_member, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.propose_add_board_member(managed_address!(&new_board_member));
        })
        .assert_user_error(err2str(ERROR_ALREADY_BOARD_MEMBER));
}

#[test]
fn dao_add_member_and_change_board_quorum_test() {
    DebugApi::dummy();
    let big_zero = rust_biguint!(0u64);
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
    let new_board_member = sc_setup.setup_new_user(1u64);
    let new_quorum = 2;
    let mut action_id: usize = 0;
    // add new board member
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            action_id = sc.propose_add_board_member(managed_address!(&new_board_member));
        })
        .assert_ok();
    // perform action - add new board member
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.perform_action(action_id);
        })
        .assert_ok();
    // propose change board quorum
    sc_setup.blockchain_wrapper
        .execute_tx(&new_board_member, &sc_setup.dao_wrapper, &big_zero, |sc| {
            action_id = sc.propose_change_board_quorum(new_quorum);
        })
        .assert_ok();
    // perform action - change board quorum
    sc_setup.blockchain_wrapper
        .execute_tx(&new_board_member, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.perform_action(action_id);
        })
        .assert_ok();
}

#[test]
fn dao_change_board_quorum_fail_test() {
    DebugApi::dummy();
    let big_zero = rust_biguint!(0u64);
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
    let new_quorum = 2;
    let mut action_id: usize = 0;
    // propose change board quorum - should fail since we only have one board member (the owner)
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            action_id = sc.propose_change_board_quorum(new_quorum);
        })
        .assert_user_error(err2str(ERROR_QUORUM_TOO_HIGH));
    // perform action - should fail since the proposal was not successful
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.perform_action(action_id);
        })
        .assert_user_error(err2str(ERROR_ACTION_NOT_FOUND));
}

#[test]
fn dao_remove_last_board_member_fail_test() {
    DebugApi::dummy();
    let big_zero = rust_biguint!(0u64);
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
    // propose remove board member - should fail since we only have one board member (the owner)
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.propose_remove_user(managed_address!(&owner));
        })
        .assert_user_error(err2str(ERROR_LAST_BOARD_MEMBER));
}

#[test]
fn dao_board_quorum_decrease_test() {
    DebugApi::dummy();
    let big_zero = rust_biguint!(0u64);
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
    let new_board_member = sc_setup.setup_new_user(1u64);
    let new_quorum = 2;
    let (mut action_id, mut action2_id) = (0, 0);
    // add new board member
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            action_id = sc.propose_add_board_member(managed_address!(&new_board_member));
        })
        .assert_ok();
    // perform action - add new board member
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.perform_action(action_id);
        })
        .assert_ok();
    // change board quorum
    sc_setup.blockchain_wrapper
        .execute_tx(&new_board_member, &sc_setup.dao_wrapper, &big_zero, |sc| {
            action_id = sc.propose_change_board_quorum(new_quorum);
        })
        .assert_ok();
    // perform action - change board quorum
    sc_setup.blockchain_wrapper
        .execute_tx(&new_board_member, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.perform_action(action_id);
        })
        .assert_ok();
    // propose remove new board member
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            action_id = sc.propose_remove_user(managed_address!(&new_board_member));
        })
        .assert_ok();
    // propose remove board member (the owner)
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            action2_id = sc.propose_remove_user(managed_address!(&owner));
        })
        .assert_ok();
    // perform action - remove new board member
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.perform_action(action_id);
        })
        .assert_ok();
    // perform action - remove owner should fail since we are left with only one board member
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.perform_action(action2_id);
        })
        .assert_user_error(err2str(ERROR_LAST_BOARD_MEMBER));
    // unsign second action
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.unsign(action2_id);
        })
        .assert_ok();
    // discard second action
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.discard_action(action2_id);
        })
        .assert_ok();
}

#[test]
fn dao_add_voting_token_test() {
    DebugApi::dummy();
    let big_zero = rust_biguint!(0u64);
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
    let mut action_id: usize = 0;
    let weight = exp18(2);
    // propose add voting token - should fail since we already have the governance token as voting token
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.propose_add_voting_token(managed_token_id!(DAO_GOVERNANCE_TOKEN_ID), weight.clone().into());
        })
        .assert_user_error(err2str(ERROR_TOKEN_ALREADY_EXISTS));
    // propose add voting token
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            action_id = sc.propose_add_voting_token(managed_token_id!(FRANCHISE1_GOVERNANCE_TOKEN_ID), weight.into());
        })
        .assert_ok();
    // perform action - add voting token
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.perform_action(action_id);
        })
        .assert_ok();
}

#[test]
fn dao_remove_all_voting_tokens_test() {
    DebugApi::dummy();
    let big_zero = rust_biguint!(0u64);
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
    let mut action_id: usize = 0;
    // propose remove the only voting token
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            action_id = sc.propose_remove_voting_token(managed_token_id!(DAO_GOVERNANCE_TOKEN_ID));
        })
        .assert_ok();
    // perform action - remove the only voting token
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.perform_action(action_id);
        })
        .assert_ok();
    // check if sc is disabled
    sc_setup.blockchain_wrapper
        .execute_query(&sc_setup.dao_wrapper, |sc| {
            assert!(sc.state().get() == State::Inactive);
        })
        .assert_ok();
}
