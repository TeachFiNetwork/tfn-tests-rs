#![allow(clippy::too_many_arguments)]

use multiversx_sc::types::Address;
use tfn_platform::common::config::SubscriberDetails;
use crate::contracts_setup::*;
use multiversx_sc_scenario::{managed_address, managed_buffer, managed_token_id, num_bigint, rust_biguint, DebugApi};
use tfn_dao::{multisig::MultisigModule, common::config::*, *};

impl<
    TFNDAOContractObjBuilder,
    TFNDEXContractObjBuilder,
    TFNPlatformContractObjBuilder,
    TFNFranchiseDAOContractObjBuilder,
    TFNEmployeeContractObjBuilder,
    TFNStudentContractObjBuilder,
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
    TFNEmployeeContractObjBuilder,
    TFNStudentContractObjBuilder,
    TFNLaunchpadContractObjBuilder,
    TFNStakingContractObjBuilder,
    TFNTestLaunchpadContractObjBuilder,
    TFNTestStakingContractObjBuilder,
    TFNTestDEXContractObjBuilder,
    TFNNFTMarketplaceContractObjBuilder,
    TFNDigitalIdentityContractObjBuilder,
>
where
    TFNDAOContractObjBuilder: 'static + Copy + Fn() -> tfn_dao::ContractObj<DebugApi>,
    TFNDEXContractObjBuilder: 'static + Copy + Fn() -> tfn_dex::ContractObj<DebugApi>,
    TFNPlatformContractObjBuilder: 'static + Copy + Fn() -> tfn_platform::ContractObj<DebugApi>,
    TFNFranchiseDAOContractObjBuilder: 'static + Copy + Fn() -> tfn_franchise_dao::ContractObj<DebugApi>,
    TFNEmployeeContractObjBuilder: 'static + Copy + Fn() -> tfn_employee::ContractObj<DebugApi>,
    TFNStudentContractObjBuilder: 'static + Copy + Fn() -> tfn_student::ContractObj<DebugApi>,
    TFNLaunchpadContractObjBuilder: 'static + Copy + Fn() -> tfn_launchpad::ContractObj<DebugApi>,
    TFNStakingContractObjBuilder: 'static + Copy + Fn() -> tfn_staking::ContractObj<DebugApi>,
    TFNTestLaunchpadContractObjBuilder: 'static + Copy + Fn() -> tfn_test_launchpad::ContractObj<DebugApi>,
    TFNTestStakingContractObjBuilder: 'static + Copy + Fn() -> tfn_test_staking::ContractObj<DebugApi>,
    TFNTestDEXContractObjBuilder: 'static + Copy + Fn() -> tfn_test_dex::ContractObj<DebugApi>,
    TFNNFTMarketplaceContractObjBuilder: 'static + Copy + Fn() -> tfn_nft_marketplace::ContractObj<DebugApi>,
    TFNDigitalIdentityContractObjBuilder: 'static + Copy + Fn() -> tfn_digital_identity::ContractObj<DebugApi>,
{
    // multisig
    pub fn dao_propose_add_board_member(
        &mut self,
        caller: &Address,
        new_member: &Address,
        err: Option<&[u8]>,
    ) -> usize {
        let mut action_id = 0;
        let result = self.blockchain_wrapper
            .execute_tx(caller, &self.dao_wrapper, &rust_biguint!(0u64), |sc| {
                action_id = sc.propose_add_board_member(managed_address!(new_member));
            });
        self.handle_error(&result, err);

        action_id
    }

    pub fn dao_propose_remove_board_member(
        &mut self,
        caller: &Address,
        member_to_remove: &Address,
        err: Option<&[u8]>,
    ) -> usize {
        let mut action_id = 0;
        let result = self.blockchain_wrapper
            .execute_tx(caller, &self.dao_wrapper, &rust_biguint!(0u64), |sc| {
                action_id = sc.propose_remove_user(managed_address!(member_to_remove));
            });
        self.handle_error(&result, err);

        action_id
    }

    pub fn dao_propose_change_board_quorum(
        &mut self,
        caller: &Address,
        new_quorum: usize,
        err: Option<&[u8]>,
    ) -> usize {
        let mut action_id = 0;
        let result = self.blockchain_wrapper
            .execute_tx(caller, &self.dao_wrapper, &rust_biguint!(0u64), |sc| {
                action_id = sc.propose_change_board_quorum(new_quorum);
            });
        self.handle_error(&result, err);

        action_id
    }

    pub fn dao_sign_action(
        &mut self,
        caller: &Address,
        action_id: usize,
        err: Option<&[u8]>,
    ) {
        let result = self.blockchain_wrapper
            .execute_tx(caller, &self.dao_wrapper, &rust_biguint!(0u64), |sc| {
                sc.sign(action_id);
            });
        self.handle_error(&result, err);
    }

    pub fn dao_unsign_action(
        &mut self,
        caller: &Address,
        action_id: usize,
        err: Option<&[u8]>,
    ) {
        let result = self.blockchain_wrapper
            .execute_tx(caller, &self.dao_wrapper, &rust_biguint!(0u64), |sc| {
                sc.unsign(action_id);
            });
        self.handle_error(&result, err);
    }

    pub fn dao_discard_action(
        &mut self,
        caller: &Address,
        action_id: usize,
        err: Option<&[u8]>,
    ) {
        let result = self.blockchain_wrapper
            .execute_tx(caller, &self.dao_wrapper, &rust_biguint!(0u64), |sc| {
                sc.discard_action(action_id);
            });
        self.handle_error(&result, err);
    }

    pub fn dao_propose_add_voting_token(
        &mut self,
        caller: &Address,
        new_token: &str,
        weight: &num_bigint::BigUint,
        err: Option<&[u8]>,
    ) -> usize {
        let mut action_id = 0;
        let result = self.blockchain_wrapper
            .execute_tx(caller, &self.dao_wrapper, &rust_biguint!(0u64), |sc| {
                action_id = sc.propose_add_voting_token(managed_token_id!(new_token), weight.into());
            });
        self.handle_error(&result, err);

        action_id
    }

    pub fn dao_propose_remove_voting_token(
        &mut self,
        caller: &Address,
        token_to_remove: &str,
        err: Option<&[u8]>,
    ) -> usize {
        let mut action_id = 0;
        let result = self.blockchain_wrapper
            .execute_tx(caller, &self.dao_wrapper, &rust_biguint!(0u64), |sc| {
                action_id = sc.propose_remove_voting_token(managed_token_id!(token_to_remove));
            });
        self.handle_error(&result, err);

        action_id
    }

    pub fn dao_propose_delete_proposal(
        &mut self,
        caller: &Address,
        proposal_id: u64,
        err: Option<&[u8]>,
    ) -> usize {
        let mut action_id = 0;
        let result = self.blockchain_wrapper
            .execute_tx(caller, &self.dao_wrapper, &rust_biguint!(0u64), |sc| {
                action_id = sc.propose_delete_proposal(proposal_id);
            });
        self.handle_error(&result, err);

        action_id
    }

    pub fn dao_perform_action(
        &mut self,
        caller: &Address,
        action_id: usize,
        err: Option<&[u8]>,
    ) {
        let result = self.blockchain_wrapper
            .execute_tx(caller, &self.dao_wrapper, &rust_biguint!(0u64), |sc| {
                sc.perform_action_endpoint(action_id);
            });
        self.handle_error(&result, err);
    }

    // DAO
    pub fn dao_propose_new_launchpad(
        &mut self,
        caller: &Address,
        title: &str,
        description: &str,
        kyc_enforced: bool,
        token: &str,
        payment_token: &str,
        price: &num_bigint::BigUint,
        min_buy_amount: &num_bigint::BigUint,
        max_buy_amount: &num_bigint::BigUint,
        start_time: u64,
        end_time: u64,
        err: Option<&[u8]>,
    ) -> u64 {
        let mut launchpad_id = 0;
        let result = self.blockchain_wrapper
            .execute_tx(caller, &self.dao_wrapper, &rust_biguint!(0u64), |sc| {
                launchpad_id = sc.propose_new_launchpad(
                    managed_buffer!(title.as_bytes()),
                    managed_buffer!(description.as_bytes()),
                    LaunchpadProposal{
                        details: SubscriberDetails {
                            name: managed_buffer!(b""),
                            description: managed_buffer!(b""),
                            logo: managed_buffer!(b""),
                            card: managed_buffer!(b""),
                            website: managed_buffer!(b""),
                            email: managed_buffer!(b""),
                            twitter: managed_buffer!(b""),
                            telegram: managed_buffer!(b""),
                        },
                        kyc_enforced,
                        token: managed_token_id!(token),
                        payment_token: managed_token_id!(payment_token),
                        price: price.into(),
                        min_buy_amount: min_buy_amount.into(),
                        max_buy_amount: max_buy_amount.into(),
                        start_time,
                        end_time,
                    },
                );
            });
        self.handle_error(&result, err);

        launchpad_id
    }

    pub fn dao_upvote(
        &mut self,
        caller: &Address,
        proposal_id: u64,
        token: &str,
        amount: &num_bigint::BigUint,
    ) {
        self.blockchain_wrapper
            .execute_esdt_transfer(
                caller,
                &self.dao_wrapper,
                token.as_bytes(),
                0,
                amount,
                |sc| {
                    sc.upvote(proposal_id);
                },
            )
            .assert_ok();
    }

    pub fn dao_downvote(
        &mut self,
        caller: &Address,
        proposal_id: u64,
        token: &str,
        amount: &num_bigint::BigUint,
    ) {
        self.blockchain_wrapper
            .execute_esdt_transfer(
                caller,
                &self.dao_wrapper,
                token.as_bytes(),
                0,
                amount,
                |sc| {
                    sc.downvote(proposal_id);
                },
            )
            .assert_ok();
    }

    pub fn dao_redeem(
        &mut self,
        caller: &Address,
        proposal_id: u64,
        err: Option<&[u8]>,
    ) {
        let result = self.blockchain_wrapper
            .execute_tx(caller, &self.dao_wrapper, &rust_biguint!(0u64), |sc| {
                sc.redeem(proposal_id);
            });
        self.handle_error(&result, err);
    }

    pub fn dao_execute_proposal(
        &mut self,
        caller: &Address,
        proposal_id: u64,
        err: Option<&[u8]>,
    ) {
        let result = self.blockchain_wrapper
            .execute_tx(caller, &self.dao_wrapper, &rust_biguint!(0u64), |sc| {
                sc.execute(proposal_id);
            });
        self.handle_error(&result, err);
    }

    // checks
    pub fn dao_check_state(
        &mut self,
        expected_state: State,
    ) {
        self.blockchain_wrapper
            .execute_query(&self.dao_wrapper, |sc| {
                assert!(sc.state().get() == expected_state);
            })
            .assert_ok();
    }

    pub fn dao_check_last_proposal_id(
        &mut self,
        expected_value: u64,
    ) {
        self.blockchain_wrapper
            .execute_query(&self.dao_wrapper, |sc| {
                assert!(sc.last_proposal_id().get() == expected_value);
            })
            .assert_ok();
    }

    pub fn dao_check_franchise_deployed(
        &mut self,
        franchise_address: &Address,
    ) {
        self.blockchain_wrapper
            .execute_query(&self.dao_wrapper, |sc| {
                let mut found = false;
                let franchises = sc.franchises().get();
                for franchise in franchises.into_iter() {
                    if &Address::new(franchise.to_byte_array()) == franchise_address {
                        found = true;
                        break;
                    }
                }
                assert!(found);
            })
            .assert_ok();
    }
}
