use multiversx_sc_scenario::{rust_biguint, DebugApi};

use tfn_dao::common::errors::*;
use crate::{consts::*, contracts_interactions::common::*, contracts_setup::TFNContractSetup};

#[test]
fn dao_propose_new_launchpad_test() {
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
    // propose new launchpad
    let proposal_id = sc_setup.dao_propose_new_launchpad(
        &owner,
        "Launchpad proposal",
        "Launchpad proposal description",
        true,
        FRANCHISE1_GOVERNANCE_TOKEN_ID,
        PAYMENT_TOKEN_ID,
        &rust_biguint!(25).pow(PAYMENT_TOKEN_DECIMALS),
        &exp18(10),
        &exp18(100),
        DAO_VOTING_PERIOD + 1,
        DAO_VOTING_PERIOD + 11,
        None,
    );
    // propose delete proposal
    let action_id = sc_setup.dao_propose_delete_proposal(&owner, proposal_id, None);
    // perform action - delete proposal
    sc_setup.dao_perform_action(&owner, action_id, None);
    // check the last proposal id is back to 0 since we deleted the last proposal
    sc_setup.dao_check_last_proposal_id(1);
}

#[test]
fn dao_propose_new_launchpad_and_vote_test() {
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
    let vote_amount = exp18(1);
    sc_setup.blockchain_wrapper
        .set_esdt_balance(&sc_setup.owner, DAO_GOVERNANCE_TOKEN_ID.as_bytes(), &vote_amount);
    // propose new launchpad
    let proposal_id = sc_setup.dao_propose_new_launchpad(
        &owner,
        "Launchpad proposal",
        "Launchpad proposal description",
        true,
        FRANCHISE1_GOVERNANCE_TOKEN_ID,
        PAYMENT_TOKEN_ID,
        &rust_biguint!(25).pow(PAYMENT_TOKEN_DECIMALS),
        &exp18(10),
        &exp18(100),
        DAO_VOTING_PERIOD + 1,
        DAO_VOTING_PERIOD + 11,
        None,
    );
    // upvote on the proposal
    sc_setup.dao_upvote(&owner, proposal_id, DAO_GOVERNANCE_TOKEN_ID, &vote_amount);
    // propose delete proposal should fail since we have voted on it
    sc_setup.dao_propose_delete_proposal(&owner, proposal_id, Some(ERROR_PROPOSAL_VOTERS_NOT_EMPTY));
    // advance the time to end the voting period
    sc_setup.blockchain_wrapper.set_block_timestamp(DAO_VOTING_PERIOD);
    // redeem tokens
    sc_setup.dao_redeem(&owner, proposal_id, None);
    // check token balance
    sc_setup.blockchain_wrapper
        .check_esdt_balance(&sc_setup.owner, DAO_GOVERNANCE_TOKEN_ID.as_bytes(), &vote_amount);
    // propose delete proposal
    let action_id = sc_setup.dao_propose_delete_proposal(&owner, proposal_id, None);
    // perform action - delete proposal
    sc_setup.dao_perform_action(&owner, action_id, None);
}


#[test]
fn dao_execute_new_launchpad_test() {
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
    let vote_amount = exp18(DAO_QUORUM);
    sc_setup.blockchain_wrapper
        .set_esdt_balance(&sc_setup.owner, DAO_GOVERNANCE_TOKEN_ID.as_bytes(), &vote_amount);
    let mut start_time = DAO_VOTING_PERIOD + 1;
    let mut end_time = start_time + 10;
    // propose new launchpad
    let proposal_id = sc_setup.dao_propose_new_launchpad(
        &owner,
        "Launchpad proposal",
        "Launchpad proposal description",
        true,
        FRANCHISE1_GOVERNANCE_TOKEN_ID,
        PAYMENT_TOKEN_ID,
        &rust_biguint!(25).pow(PAYMENT_TOKEN_DECIMALS),
        &exp18(10),
        &exp18(100),
        start_time,
        end_time,
        None,
    );
    // upvote on the proposal
    sc_setup.dao_upvote(&owner, proposal_id, DAO_GOVERNANCE_TOKEN_ID, &vote_amount);
    // advance the time to end the voting period
    sc_setup.blockchain_wrapper.set_block_timestamp(DAO_VOTING_PERIOD);
    start_time += DAO_VOTING_PERIOD;
    end_time += DAO_VOTING_PERIOD;
    // execute the proposal
    sc_setup.dao_execute_proposal(&owner, proposal_id, None);
    // check the launchpad contract
    sc_setup.launchpad_check_last_launchpad_id(2);
    // propose same launchpad should fail as it is already on launchpad
    sc_setup.dao_propose_new_launchpad(
        &owner,
        "Launchpad proposal",
        "Launchpad proposal description",
        true,
        FRANCHISE1_GOVERNANCE_TOKEN_ID,
        PAYMENT_TOKEN_ID,
        &rust_biguint!(25).pow(PAYMENT_TOKEN_DECIMALS),
        &exp18(10),
        &exp18(100),
        start_time,
        end_time,
        Some(ERROR_TOKEN_ALREADY_LAUNCHED),
    );
}
