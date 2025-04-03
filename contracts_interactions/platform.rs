use multiversx_sc::{imports::OptionalValue, types::Address};
use multiversx_sc_scenario::{managed_address, num_bigint, rust_biguint, DebugApi};

use crate::contracts_setup::TFNContractSetup;
use tfn_platform::{common::config::*, TFNPlatformContract};

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
    pub fn platform_subscribe(
        &mut self,
        caller: &Address,
        token: &str,
        amount: &num_bigint::BigUint,
        err: Option<&[u8]>,
    ) {
        let result = self.blockchain_wrapper
            .execute_esdt_transfer(
                caller,
                &self.platform_wrapper,
                token.as_bytes(),
                0,
                amount,
                |sc| {
                sc.subscribe(OptionalValue::None);
            });
        self.handle_error(&result, err);
    }

    pub fn platform_whitelist_address(
        &mut self,
        caller: &Address,
        user: &Address,
        err: Option<&[u8]>,
    ) {
        let result = self.blockchain_wrapper
            .execute_tx(caller, &self.platform_wrapper, &rust_biguint!(0u64), |sc| {
                sc.whitelist_address(managed_address!(user));
            });
        self.handle_error(&result, err);
    }

    pub fn platform_remove_address(
        &mut self,
        caller: &Address,
        user: &Address,
        err: Option<&[u8]>,
    ) {
        let result = self.blockchain_wrapper
            .execute_tx(caller, &self.platform_wrapper, &rust_biguint!(0u64), |sc| {
                sc.remove_address(managed_address!(user));
            });
        self.handle_error(&result, err);
    }

    // checks
    pub fn platform_check_is_subscribed(
        &mut self,
        subscriber: &Address,
    ) {
        self.blockchain_wrapper
            .execute_query(&self.platform_wrapper, |sc| {
                assert!(sc.get_subscriber_id_by_address(&managed_address!(subscriber)).is_some());
            })
            .assert_ok();
    }
}
