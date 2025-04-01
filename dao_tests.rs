use multiversx_sc_scenario::{managed_buffer, managed_token_id, rust_biguint, DebugApi};

use tfn_dao::{common::{config::*, errors::*}, multisig::MultisigModule, TFNDAOContract};
use tfn_launchpad::common::config::ConfigModule as _;

use crate::{consts::*, contracts_setup::TFNContractSetup, *};

#[test]
fn dao_propose_new_launchpad_test() {
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
    let mut action_id = 0;
    let price = rust_biguint!(25).pow(PAYMENT_TOKEN_DECIMALS);
    let min_buy_amount = exp18(10);
    let max_buy_amount = exp18(100);
    let title = "Launchpad proposal".as_bytes().to_vec();
    let description = "Launchpad proposal description".as_bytes().to_vec();
    let start_time = DAO_VOTING_PERIOD + 1;
    let end_time = start_time + 10;
    // propose new launchpad
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.propose_new_launchpad(
                managed_buffer!(&title),
                managed_buffer!(&description),
                LaunchpadProposal{
                    kyc_enforced: true,
                    token: managed_token_id!(FRANCHISE1_GOVERNANCE_TOKEN_ID),
                    payment_token: managed_token_id!(PAYMENT_TOKEN_ID),
                    price: price.into(),
                    min_buy_amount: min_buy_amount.into(),
                    max_buy_amount: max_buy_amount.into(),
                    start_time,
                    end_time,
                },
            );
        })
        .assert_ok();
    // propose delete proposal
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            action_id = sc.propose_delete_proposal(0);
        })
        .assert_ok();
    // perform action - delete proposal
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.perform_action(action_id);
        })
        .assert_ok();
    // check the last proposal id is back to 0 since we deleted the last proposal
    sc_setup.blockchain_wrapper
        .execute_query(&sc_setup.dao_wrapper, |sc| {
            assert!(sc.last_proposal_id().is_empty());
        })
        .assert_ok();
}

#[test]
fn dao_propose_new_launchpad_and_vote_test() {
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
    let mut action_id = 0;
    let vote_amount = exp18(1);
    sc_setup.blockchain_wrapper
        .set_esdt_balance(&sc_setup.owner, DAO_GOVERNANCE_TOKEN_ID.as_bytes(), &vote_amount);
    let price = rust_biguint!(25).pow(PAYMENT_TOKEN_DECIMALS);
    let min_buy_amount = exp18(10);
    let max_buy_amount = exp18(100);
    let title = "Launchpad proposal".as_bytes().to_vec();
    let description = "Launchpad proposal description".as_bytes().to_vec();
    let start_time = DAO_VOTING_PERIOD + 1;
    let end_time = start_time + 10;
    let mut proposal_id = 0;
    // propose new launchpad
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            proposal_id = sc.propose_new_launchpad(
                managed_buffer!(&title),
                managed_buffer!(&description),
                LaunchpadProposal{
                    kyc_enforced: true,
                    token: managed_token_id!(FRANCHISE1_GOVERNANCE_TOKEN_ID),
                    payment_token: managed_token_id!(PAYMENT_TOKEN_ID),
                    price: price.into(),
                    min_buy_amount: min_buy_amount.into(),
                    max_buy_amount: max_buy_amount.into(),
                    start_time,
                    end_time,
                },
            );
        })
        .assert_ok();
    // upvote on the proposal
    sc_setup.blockchain_wrapper
        .execute_esdt_transfer(
            &sc_setup.owner,
            &sc_setup.dao_wrapper,
            DAO_GOVERNANCE_TOKEN_ID.as_bytes(),
            0,
            &vote_amount,
            |sc| {
            sc.vote(proposal_id, VoteType::Upvote);
        })
        .assert_ok();
    // propose delete proposal should fail since we have voted on it
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            action_id = sc.propose_delete_proposal(proposal_id);
        })
        .assert_user_error(err2str(ERROR_PROPOSAL_VOTERS_NOT_EMPTY));
    // advance the time to end the voting period
    sc_setup.blockchain_wrapper.set_block_timestamp(DAO_VOTING_PERIOD);
    // redeem tokens
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.redeem(proposal_id);
        })
        .assert_ok();
    // check token balance
    sc_setup.blockchain_wrapper
        .check_esdt_balance(&sc_setup.owner, DAO_GOVERNANCE_TOKEN_ID.as_bytes(), &vote_amount);
    // propose delete proposal
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            action_id = sc.propose_delete_proposal(proposal_id);
        })
        .assert_ok();
    // perform action - delete proposal
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.perform_action(action_id);
        })
        .assert_ok();
}


#[test]
fn dao_execute_new_launchpad_test() {
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
    let vote_amount = exp18(DAO_QUORUM);
    sc_setup.blockchain_wrapper
        .set_esdt_balance(&sc_setup.owner, DAO_GOVERNANCE_TOKEN_ID.as_bytes(), &vote_amount);
    let price = rust_biguint!(25).pow(PAYMENT_TOKEN_DECIMALS);
    let min_buy_amount = exp18(10);
    let max_buy_amount = exp18(100);
    let title = "Launchpad proposal".as_bytes().to_vec();
    let description = "Launchpad proposal description".as_bytes().to_vec();
    let mut start_time = DAO_VOTING_PERIOD + 1;
    let mut end_time = start_time + 10;
    let mut proposal_id = 0;
    // propose new launchpad
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            proposal_id = sc.propose_new_launchpad(
                managed_buffer!(&title),
                managed_buffer!(&description),
                LaunchpadProposal{
                    kyc_enforced: true,
                    token: managed_token_id!(FRANCHISE1_GOVERNANCE_TOKEN_ID),
                    payment_token: managed_token_id!(PAYMENT_TOKEN_ID),
                    price: price.clone().into(),
                    min_buy_amount: min_buy_amount.clone().into(),
                    max_buy_amount: max_buy_amount.clone().into(),
                    start_time,
                    end_time,
                },
            );
        })
        .assert_ok();
    // upvote on the proposal
    sc_setup.blockchain_wrapper
        .execute_esdt_transfer(
            &sc_setup.owner,
            &sc_setup.dao_wrapper,
            DAO_GOVERNANCE_TOKEN_ID.as_bytes(),
            0,
            &vote_amount,
            |sc| {
            sc.vote(proposal_id, VoteType::Upvote);
        })
        .assert_ok();
    // advance the time to end the voting period
    sc_setup.blockchain_wrapper.set_block_timestamp(DAO_VOTING_PERIOD);
    start_time += DAO_VOTING_PERIOD;
    end_time += DAO_VOTING_PERIOD;
    // execute the proposal
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.execute(proposal_id);
        })
        .assert_ok();
    // check the launchpad contract
    sc_setup.blockchain_wrapper
        .execute_query(&sc_setup.launchpad_wrapper, |sc| {
            assert!(!sc.last_launchpad_id().is_empty());
        })
        .assert_ok();
    // propose same launchpad should fail as it is already on launchpad
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.propose_new_launchpad(
                managed_buffer!(&title),
                managed_buffer!(&description),
                LaunchpadProposal{
                    kyc_enforced: true,
                    token: managed_token_id!(FRANCHISE1_GOVERNANCE_TOKEN_ID),
                    payment_token: managed_token_id!(PAYMENT_TOKEN_ID),
                    price: price.into(),
                    min_buy_amount: min_buy_amount.into(),
                    max_buy_amount: max_buy_amount.into(),
                    start_time,
                    end_time,
                },
            );
        })
        .assert_user_error(err2str(ERROR_TOKEN_ALREADY_LAUNCHED));
}
