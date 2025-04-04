use multiversx_sc_scenario::DebugApi;
use tfn_dao::{common::errors::*, common::config::*};

use crate::{consts::*, contracts_interactions::common::*, contracts_setup::TFNContractSetup};

#[test]
fn dao_add_new_board_member_test() {
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
    let new_board_member = sc_setup.setup_new_user(1u64);
    // new member propose add new board member - should fail
    sc_setup.dao_propose_add_board_member(&new_board_member, &new_board_member, Some(ERROR_ONLY_BOARD_MEMBERS));
    // owner propose add new board member
    let action_id = sc_setup.dao_propose_add_board_member(&owner, &new_board_member, None);
    // perform action - add new board member
    sc_setup.dao_perform_action(&owner, action_id, None);
    // propose add new board member again - should fail
    sc_setup.dao_propose_add_board_member(&new_board_member, &new_board_member, Some(ERROR_ALREADY_BOARD_MEMBER));
}

#[test]
fn dao_add_member_and_change_board_quorum_test() {
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
    let new_board_member = sc_setup.setup_new_user(1u64);
    let new_quorum = 2;
    // add new board member
    let mut action_id = sc_setup.dao_propose_add_board_member(&owner, &new_board_member, None);
    // perform action - add new board member
    sc_setup.dao_perform_action(&owner, action_id, None);
    // propose change board quorum
    action_id = sc_setup.dao_propose_change_board_quorum(&new_board_member, new_quorum, None);
    // perform action - change board quorum
    sc_setup.dao_perform_action(&new_board_member, action_id, None);
}

#[test]
fn dao_change_board_quorum_fail_test() {
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
    let new_quorum = 2;
    // propose change board quorum - should fail since we only have one board member (the owner)
    let action_id = sc_setup.dao_propose_change_board_quorum(&owner, new_quorum, Some(ERROR_QUORUM_TOO_HIGH));
    // perform action - should fail since the proposal was not successful
    sc_setup.dao_perform_action(&owner, action_id, Some(ERROR_ACTION_NOT_FOUND));
}

#[test]
fn dao_remove_last_board_member_fail_test() {
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
    // propose remove board member - should fail since we only have one board member (the owner)
    sc_setup.dao_propose_remove_board_member(&owner, &owner, Some(ERROR_LAST_BOARD_MEMBER));
}

#[test]
fn dao_board_quorum_decrease_test() {
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
    let new_board_member = sc_setup.setup_new_user(1u64);
    let new_quorum = 2;
    // add new board member
    let mut action_id = sc_setup.dao_propose_add_board_member(&owner, &new_board_member, None);
    // perform action - add new board member
    sc_setup.dao_perform_action(&owner, action_id, None);
    // change board quorum
    action_id = sc_setup.dao_propose_change_board_quorum(&new_board_member, new_quorum, None);
    // perform action - change board quorum
    sc_setup.dao_perform_action(&new_board_member, action_id, None);
    // propose remove new board member
    action_id = sc_setup.dao_propose_remove_board_member(&owner, &new_board_member, None);
    // propose remove board member (the owner)
    let action2_id = sc_setup.dao_propose_remove_board_member(&owner, &owner, None);
    // perform action - remove new board member should fail since we don't have quorum
    sc_setup.dao_perform_action(&owner, action_id, Some(ERROR_QUORUM_NOT_REACHED));
    // sign action - remove new board member
    sc_setup.dao_sign_action(&new_board_member, action_id, None);
    // perform action - remove new board member
    sc_setup.dao_perform_action(&owner, action_id, None);
    // perform action - remove owner should fail since we are left with only one board member
    sc_setup.dao_perform_action(&owner, action2_id, Some(ERROR_LAST_BOARD_MEMBER));
    // unsign second action
    sc_setup.dao_unsign_action(&owner, action2_id, None);
    // discard second action
    sc_setup.dao_discard_action(&owner, action2_id, None);
}

#[test]
fn dao_add_voting_token_test() {
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
    let weight = exp18(2);
    // propose add voting token - should fail since we already have the governance token as voting token
    sc_setup.dao_propose_add_voting_token(&owner, DAO_GOVERNANCE_TOKEN_ID, &weight, Some(ERROR_TOKEN_ALREADY_EXISTS));
    // propose add voting token
    let action_id = sc_setup.dao_propose_add_voting_token(&owner, FRANCHISE1_GOVERNANCE_TOKEN_ID, &weight, None);
    // perform action - add voting token
    sc_setup.dao_perform_action(&owner, action_id, None);
    // perform same action again should fail since the action should not exist anymore
    sc_setup.dao_perform_action(&owner, action_id, Some(ERROR_ACTION_NOT_FOUND));
}

#[test]
fn dao_remove_all_voting_tokens_test() {
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
    // propose remove the only voting token
    let action_id = sc_setup.dao_propose_remove_voting_token(&owner, DAO_GOVERNANCE_TOKEN_ID, None);
    // perform action - remove the only voting token
    sc_setup.dao_perform_action(&owner, action_id, None);
    // check if sc is disabled
    sc_setup.dao_check_state(State::Inactive);
}
