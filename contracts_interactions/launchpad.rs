#![allow(clippy::too_many_arguments)]

use multiversx_sc::types::Address;
use multiversx_sc_scenario::{managed_address, managed_token_id, num_bigint, rust_biguint, DebugApi};

use crate::{consts::ISSUE_TOKEN_PRICE, contracts_setup::TFNContractSetup};

use tfn_launchpad::{common::config::*, *};

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
    TFNDigitalIdentityContractObjBuilder,
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
    pub fn launchpad_new_launchpad(
        &mut self,
        caller: &Address,
        owner: &Address,
        identity_id: u64,
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
            .execute_tx(caller, &self.launchpad_wrapper, &rust_biguint!(0u64), |sc| {
                launchpad_id = sc.new_launchpad(
                    managed_address!(owner),
                    identity_id,
                    kyc_enforced,
                    managed_token_id!(token),
                    managed_token_id!(payment_token),
                    price.into(),
                    min_buy_amount.into(),
                    max_buy_amount.into(),
                    start_time,
                    end_time,
                );
            });
        self.handle_error(&result, err);

        launchpad_id
    }

    pub fn launchpad_deploy_franchise(
        &mut self,
        caller: &Address,
        launchpad_id: u64,
        err: Option<&[u8]>,
    ) -> Address {
        let mut franchise_address = Address::zero();
        let result = self.blockchain_wrapper
            .execute_tx(caller, &self.launchpad_wrapper, &rust_biguint!(ISSUE_TOKEN_PRICE), |sc| {
                let new_address = sc.deploy_franchise(launchpad_id);
                franchise_address = Address::new(new_address.to_byte_array());
            });
        self.handle_error(&result, err);

        franchise_address
    }

    pub fn launchpad_add_tokens(
        &mut self,
        caller: &Address,
        launchpad_id: u64,
        token: &str,
        amount: &num_bigint::BigUint,
        err: Option<&[u8]>,
    ) {
        let result = self.blockchain_wrapper
            .execute_esdt_transfer(
                caller,
                &self.launchpad_wrapper,
                token.as_bytes(),
                0,
                amount,
                |sc| {
                    sc.add_tokens(launchpad_id);
                });
        self.handle_error(&result, err);
    }

    pub fn launchpad_buy(
        &mut self,
        caller: &Address,
        launchpad_id: u64,
        payment_token: &str,
        amount: &num_bigint::BigUint,
        err: Option<&[u8]>,
    ) {
        let result = self.blockchain_wrapper
            .execute_esdt_transfer(
                caller,
                &self.launchpad_wrapper,
                payment_token.as_bytes(),
                0,
                amount,
                |sc| {
                sc.buy(launchpad_id);
            });
        self.handle_error(&result, err);
    }

    pub fn launchpad_whitelist_user(
        &mut self,
        caller: &Address,
        launchpad_id: u64,
        user: &Address,
        err: Option<&[u8]>,
    ) {
        let result = self.blockchain_wrapper
            .execute_tx(caller, &self.launchpad_wrapper, &rust_biguint!(0u64), |sc| {
                sc.whitelist_user(launchpad_id, managed_address!(user));
            });
        self.handle_error(&result, err);
    }

    pub fn launchpad_cancel_launchpad(
        &mut self,
        caller: &Address,
        launchpad_id: u64,
        err: Option<&[u8]>,
    ) {
        let result = self.blockchain_wrapper
            .execute_tx(caller, &self.launchpad_wrapper, &rust_biguint!(0u64), |sc| {
                sc.cancel_launchpad(launchpad_id);
            });
        self.handle_error(&result, err);
    }

    // checks
    pub fn launchpad_check_state(
        &mut self,
        expected_state: State,
    ) {
        self.blockchain_wrapper
            .execute_query(&self.launchpad_wrapper, |sc| {
                assert!(sc.state().get() == expected_state);
            })
            .assert_ok();
    }

    pub fn launchpad_check_last_launchpad_id(
        &mut self,
        expected_value: u64,
    ) {
        self.blockchain_wrapper
            .execute_query(&self.launchpad_wrapper, |sc| {
                assert!(sc.last_launchpad_id().get() == expected_value);
            })
            .assert_ok();
    }

    pub fn launchpad_check_launchpad_status(
        &mut self,
        launchpad_id: u64,
        expected_status: Status,
        current_time: u64,
    ) {
        self.blockchain_wrapper
            .execute_query(&self.launchpad_wrapper, |sc| {
                assert!(sc.launchpads(launchpad_id).get().get_status(current_time) == expected_status);
            })
            .assert_ok();
    }
}
