#![allow(clippy::too_many_arguments)]

use multiversx_sc::types::Address;
use multiversx_sc_scenario::{imports::TxTokenTransfer, managed_token_id, num_bigint, rust_biguint, DebugApi};

use crate::contracts_setup::TFNContractSetup;
use tfn_test_dex::{*, swap::*, liquidity::*, common::config::*};

// use super::common::DEFAULT_ROLES;

impl<
    TFNDAOContractObjBuilder,
    TFNDEXContractObjBuilder,
    TFNPlatformContractObjBuilder,
    TFNFranchiseDAOContractObjBuilder,
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
    TFNLaunchpadContractObjBuilder: 'static + Copy + Fn() -> tfn_launchpad::ContractObj<DebugApi>,
    TFNStakingContractObjBuilder: 'static + Copy + Fn() -> tfn_staking::ContractObj<DebugApi>,
    TFNTestLaunchpadContractObjBuilder: 'static + Copy + Fn() -> tfn_test_launchpad::ContractObj<DebugApi>,
    TFNTestStakingContractObjBuilder: 'static + Copy + Fn() -> tfn_test_staking::ContractObj<DebugApi>,
    TFNTestDEXContractObjBuilder: 'static + Copy + Fn() -> tfn_test_dex::ContractObj<DebugApi>,
    TFNNFTMarketplaceContractObjBuilder: 'static + Copy + Fn() -> tfn_nft_marketplace::ContractObj<DebugApi>,
    TFNDigitalIdentityContractObjBuilder: 'static + Copy + Fn() -> tfn_digital_identity::ContractObj<DebugApi>,
{
    pub fn test_dex_create_pair(
        &mut self,
        caller: &Address,
        token: &str,
        base_token: &str,
        lp_fee: u64,
        owner_fee: u64,
        err: Option<&[u8]>,
    ) {
        let result = self.blockchain_wrapper
            .execute_tx(caller, &self.test_dex_wrapper, &rust_biguint!(0u64), |sc| {
                sc.create_pair(
                    managed_token_id!(base_token),
                    managed_token_id!(token),
                    lp_fee,
                    owner_fee,
                );
            });
        self.handle_error(&result, err);
        // if err.is_none() {
        //     let mut lp_token = rust_biguint!(0);
        //     self.blockchain_wrapper
        //         .execute_tx(caller, &self.test_dex_wrapper, &rust_biguint!(0u64), |sc| {
        //             let token = sc.test_create_pair(
        //                 managed_token_id!(base_token),
        //                 managed_token_id!(token),
        //                 decimals,
        //                 lp_fee,
        //                 owner_fee,
        //             );
        //             lp_token = num_bigint::BigUint::from_bytes_be(token.to_string().as_bytes());
        //         })
        //         .assert_ok();
        //     self.blockchain_wrapper
        //         .set_esdt_local_roles(self.test_dex_wrapper.address_ref(), lp_token.to_bytes_be().as_slice(), DEFAULT_ROLES);
        // }
    }

    pub fn test_dex_add_liquidity(
        &mut self,
        caller: &Address,
        token: &str,
        token_amount: &num_bigint::BigUint,
        base_token: &str,
        base_token_amount: &num_bigint::BigUint,
        err: Option<&[u8]>,
    ) {
        let transfers: Vec<TxTokenTransfer> = vec![
            TxTokenTransfer { token_identifier: token.as_bytes().to_vec(), nonce: 0, value: token_amount.clone() },
            TxTokenTransfer { token_identifier: base_token.as_bytes().to_vec(), nonce: 0, value: base_token_amount.clone() },
        ];
        let result = self.blockchain_wrapper
            .execute_esdt_multi_transfer(caller, &self.test_dex_wrapper, &transfers, |sc| {
                    sc.add_liquidity();
                },
            );
        self.handle_error(&result, err);
    }

    pub fn test_dex_remove_liquidity(
        &mut self,
        caller: &Address,
        lp_token: &str,
        lp_token_amount: &num_bigint::BigUint,
        err: Option<&[u8]>,
    ) {
        let result = self.blockchain_wrapper
            .execute_esdt_transfer(caller, &self.test_dex_wrapper, lp_token.as_bytes(),0,lp_token_amount, |sc| {
                    sc.remove_liquidity();
                },
            );
        self.handle_error(&result, err);
    }

    pub fn test_dex_swap_fixed_input(
        &mut self,
        caller: &Address,
        from_token: &str,
        from_token_amount: &num_bigint::BigUint,
        to_token: &str,
        min_amount_out: &num_bigint::BigUint,
        err: Option<&[u8]>,
    ) {
        let result = self.blockchain_wrapper
            .execute_esdt_transfer(caller, &self.test_dex_wrapper, from_token.as_bytes(), 0, from_token_amount, |sc| {
                sc.swap_fixed_input(
                    managed_token_id!(to_token),
                    min_amount_out.into(),
                );
            });
        self.handle_error(&result, err);
    }

    pub fn test_dex_swap_fixed_output(
        &mut self,
        caller: &Address,
        from_token: &str,
        from_token_amount: &num_bigint::BigUint,
        to_token: &str,
        amount_out_wanted: &num_bigint::BigUint,
        err: Option<&[u8]>,
    ) {
        let result = self.blockchain_wrapper
            .execute_esdt_transfer(caller, &self.test_dex_wrapper, from_token.as_bytes(), 0, from_token_amount, |sc| {
                sc.swap_fixed_output(
                    managed_token_id!(to_token),
                    amount_out_wanted.into(),
                );
            });
        self.handle_error(&result, err);
    }

    pub fn test_dex_set_pair_active(
        &mut self,
        caller: &Address,
        pair_id: usize,
        err: Option<&[u8]>,
    ) {
        let result = self.blockchain_wrapper
            .execute_tx(caller, &self.test_dex_wrapper, &rust_biguint!(0u64), |sc| {
                sc.set_pair_active(pair_id);
            });
        self.handle_error(&result, err);
    }

    pub fn test_dex_set_pair_active_no_swap(
        &mut self,
        caller: &Address,
        pair_id: usize,
        err: Option<&[u8]>,
    ) {
        let result = self.blockchain_wrapper
            .execute_tx(caller, &self.test_dex_wrapper, &rust_biguint!(0u64), |sc| {
                sc.set_pair_active_no_swap(pair_id);
            });
        self.handle_error(&result, err);
    }

    pub fn test_dex_set_pair_inactive(
        &mut self,
        caller: &Address,
        pair_id: usize,
        err: Option<&[u8]>,
    ) {
        let result = self.blockchain_wrapper
            .execute_tx(caller, &self.test_dex_wrapper, &rust_biguint!(0u64), |sc| {
                sc.set_pair_inactive(pair_id);
            });
        self.handle_error(&result, err);
    }

    // checks
    pub fn test_dex_pair_exists_by_tickers(
        &mut self,
        token: &str,
        base_token: &str,
    ) {
        self.blockchain_wrapper
            .execute_query(&self.test_dex_wrapper, |sc| {
                assert!(sc.get_pair_by_tickers(&managed_token_id!(token), &managed_token_id!(base_token)).is_some());
            })
            .assert_ok();
    }

    pub fn test_dex_check_pairs_count(
        &mut self,
        expected_count: usize,
    ) {
        self.blockchain_wrapper
            .execute_query(&self.test_dex_wrapper, |sc| {
                assert_eq!(sc.get_pairs().len(), expected_count);
            })
            .assert_ok();
    }

    pub fn test_dex_change_pair_fees(
        &mut self,
        caller: &Address,
        pair_id: usize,
        new_lp_fee: u64,
        new_owner_fee: u64,
        err: Option<&[u8]>,
    ) {
        let result = self.blockchain_wrapper
            .execute_tx(caller, &self.test_dex_wrapper, &rust_biguint!(0u64), |sc| {
                sc.change_pair_fees(pair_id, new_lp_fee, new_owner_fee);
            });
        self.handle_error(&result, err);
    }

    // views
    pub fn test_dex_get_pair_id_by_tickers(
        &mut self,
        token: &str,
        base_token: &str,
    ) -> Option<usize> {
        let mut pair_id = None;
        self.blockchain_wrapper
            .execute_query(&self.test_dex_wrapper, |sc| {
                let id = sc.get_pair_by_tickers(&managed_token_id!(token), &managed_token_id!(base_token)).unwrap().id;
                pair_id = Some(id)
            })
            .assert_ok();

        pair_id
    }

    pub fn test_dex_get_pair_lp_token_by_tickers(
        &mut self,
        token: &str,
        base_token: &str,
    ) -> Vec<u8> {
        let mut lp_token = rust_biguint!(0);
        self.blockchain_wrapper
            .execute_query(&self.test_dex_wrapper, |sc| {
                let token = sc.get_pair_by_tickers(&managed_token_id!(token), &managed_token_id!(base_token)).unwrap().lp_token;
                lp_token = num_bigint::BigUint::from_bytes_be(token.to_string().as_bytes());
            })
            .assert_ok();

        lp_token.to_bytes_be()
    }

    pub fn test_dex_get_amount_out(
        &mut self,
        token: &str,
        base_token: &str,
        amount_in: &num_bigint::BigUint,
    ) -> num_bigint::BigUint {
        let mut amount_out = rust_biguint!(0);
        self.blockchain_wrapper
            .execute_query(&self.test_dex_wrapper, |sc| {
                let amount = sc.get_amount_out_view(&managed_token_id!(token), &managed_token_id!(base_token), amount_in.into());
                amount_out = num_bigint::BigUint::from_bytes_be(amount.to_bytes_be().as_slice());
            })
            .assert_ok();

        amount_out
    }

    pub fn test_dex_get_amount_in(
        &mut self,
        token: &str,
        base_token: &str,
        amount_out: &num_bigint::BigUint,
    ) -> num_bigint::BigUint {
        let mut amount_in = rust_biguint!(0);
        self.blockchain_wrapper
            .execute_query(&self.test_dex_wrapper, |sc| {
                let amount = sc.get_amount_in_view(&managed_token_id!(token), &managed_token_id!(base_token), amount_out.into());
                amount_in = num_bigint::BigUint::from_bytes_be(amount.to_bytes_be().as_slice());
            })
            .assert_ok();

        amount_in
    }
}
