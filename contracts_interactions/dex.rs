use multiversx_sc::types::Address;
use multiversx_sc_scenario::{imports::TxTokenTransfer, managed_token_id, num_bigint, rust_biguint, DebugApi};

use crate::contracts_setup::TFNContractSetup;
use tfn_dex::{*, liquidity::*, common::config::*};

use super::common::DEFAULT_ROLES;

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
{
    pub fn dex_create_pair(
        &mut self,
        caller: &Address,
        token: &str,
        base_token: &str,
        decimals: u8,
        err: Option<&[u8]>,
    ) {
        let result = self.blockchain_wrapper
            .execute_tx(caller, &self.dex_wrapper, &rust_biguint!(0u64), |sc| {
                sc.create_pair(
                    managed_token_id!(base_token),
                    managed_token_id!(token),
                    decimals,
                );
            });
        self.handle_error(&result, err);
        if err.is_none() {
            let mut lp_token = rust_biguint!(0);
            self.blockchain_wrapper
                .execute_tx(caller, &self.dex_wrapper, &rust_biguint!(0u64), |sc| {
                    let token = sc.test_create_pair(
                        managed_token_id!(base_token),
                        managed_token_id!(token),
                        decimals,
                    );
                    lp_token = num_bigint::BigUint::from_bytes_be(token.to_string().as_bytes());
                })
                .assert_ok();
            self.blockchain_wrapper
                .set_esdt_local_roles(self.dex_wrapper.address_ref(), lp_token.to_bytes_be().as_slice(), DEFAULT_ROLES);
        }
    }

    pub fn dex_add_liquidity(
        &mut self,
        caller: &Address,
        token: &str,
        token_amount: num_bigint::BigUint,
        base_token: &str,
        base_token_amount: num_bigint::BigUint,
        err: Option<&[u8]>,
    ) {
        let transfers: Vec<TxTokenTransfer> = vec![
            TxTokenTransfer { token_identifier: token.as_bytes().to_vec(), nonce: 0, value: token_amount },
            TxTokenTransfer { token_identifier: base_token.as_bytes().to_vec(), nonce: 0, value: base_token_amount },
        ];
        let result = self.blockchain_wrapper
            .execute_esdt_multi_transfer(
                caller,
                &self.dex_wrapper,
                &transfers,
                |sc| {
                    sc.add_liquidity();
                },
            );
        self.handle_error(&result, err);
    }

    // checks
    pub fn dex_pair_exists_by_tickers(
        &mut self,
        token: &str,
        base_token: &str,
    ) {
        self.blockchain_wrapper
            .execute_query(&self.dex_wrapper, |sc| {
                assert!(sc.get_pair_by_tickers(&managed_token_id!(token), &managed_token_id!(base_token)).is_some());
            })
            .assert_ok();
    }

    pub fn dex_check_pairs_count(
        &mut self,
        expected_count: usize,
    ) {
        self.blockchain_wrapper
            .execute_query(&self.dex_wrapper, |sc| {
                assert_eq!(sc.get_pairs().len(), expected_count);
            })
            .assert_ok();
    }

    // views
    pub fn dex_get_pair_lp_token_by_tickers(
        &mut self,
        token: &str,
        base_token: &str,
    ) -> Vec<u8> {
        let mut lp_token = rust_biguint!(0);
        self.blockchain_wrapper
            .execute_query(&self.dex_wrapper, |sc| {
                let token = sc.get_pair_by_tickers(&managed_token_id!(token), &managed_token_id!(base_token)).unwrap().lp_token;
                lp_token = num_bigint::BigUint::from_bytes_be(token.to_string().as_bytes());
            })
            .assert_ok();

        lp_token.to_bytes_be()
    }
}
