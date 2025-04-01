use multiversx_sc_scenario::{managed_address, rust_biguint, DebugApi};
use tfn_dao::{common::errors::*, multisig::MultisigModule, TFNDAOContract};

use crate::{contracts_setup::TFNContractSetup, *};

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
    sc_setup.blockchain_wrapper
        .execute_tx(&new_board_member, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.propose_add_board_member(managed_address!(&new_board_member));
        })
        .assert_user_error(err2str(ERROR_ONLY_BOARD_MEMBERS));
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            action_id = sc.propose_add_board_member(managed_address!(&new_board_member));
        })
        .assert_ok();
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.perform_action(action_id);
        })
        .assert_ok();
    sc_setup.blockchain_wrapper
        .execute_tx(&new_board_member, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.propose_add_board_member(managed_address!(&new_board_member));
        })
        .assert_user_error(err2str(ERROR_ALREADY_BOARD_MEMBER));
}

#[test]
fn test_add_member_and_change_board_quorum_test() {
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
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            action_id = sc.propose_add_board_member(managed_address!(&new_board_member));
        })
        .assert_ok();
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.perform_action(action_id);
        })
        .assert_ok();

    sc_setup.blockchain_wrapper
        .execute_tx(&new_board_member, &sc_setup.dao_wrapper, &big_zero, |sc| {
            action_id = sc.propose_change_board_quorum(new_quorum);
        })
        .assert_ok();
    sc_setup.blockchain_wrapper
        .execute_tx(&new_board_member, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.perform_action(action_id);
        })
        .assert_ok();
}

#[test]
fn test_change_board_quorum_fail_test() {
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
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            action_id = sc.propose_change_board_quorum(new_quorum);
        })
        .assert_user_error(err2str(ERROR_QUORUM_TOO_HIGH));
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.perform_action(action_id);
        })
        .assert_user_error(err2str(ERROR_ACTION_NOT_FOUND));
}

#[test]
fn test_remove_last_board_member_fail_test() {
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
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.propose_remove_user(managed_address!(&owner));
        })
        .assert_user_error(err2str(ERROR_LAST_BOARD_MEMBER));
}

#[test]
fn test_board_quorum_decrease_test() {
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
    sc_setup.blockchain_wrapper
        .execute_tx(&new_board_member, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.perform_action(action_id);
        })
        .assert_ok();
    // propose remove both members
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            action_id = sc.propose_remove_user(managed_address!(&new_board_member));
        })
        .assert_ok();
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            action2_id = sc.propose_remove_user(managed_address!(&owner));
        })
        .assert_ok();
    // perform remove actions. second should fail
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.perform_action(action_id);
        })
        .assert_ok();
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.perform_action(action2_id);
        })
        .assert_user_error(err2str(ERROR_LAST_BOARD_MEMBER));
    // unsign and discard second action
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.unsign(action2_id);
        })
        .assert_ok();
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.discard_action(action2_id);
        })
        .assert_ok();
    }
