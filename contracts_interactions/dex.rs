use multiversx_sc_scenario::{managed_token_id, DebugApi};

use crate::contracts_setup::TFNContractSetup;
use tfn_dex::common::config::*;

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
}
