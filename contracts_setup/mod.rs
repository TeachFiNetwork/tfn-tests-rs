use multiversx_sc::types::Address;
use multiversx_sc_scenario::{
    managed_address, managed_biguint, managed_token_id, rust_biguint, testing_framework::*, DebugApi
};

use tfn_dao::{TFNDAOContract, common::config::ConfigModule as _};
use tfn_dex::{TFNDEXContract, common::config::ConfigModule as _};
use tfn_platform::{TFNPlatformContract, common::config::ConfigModule as _};
use tfn_franchise_dao::TFNFranchiseDAOContract;
use tfn_employee::TFNEmployeeContract;
use tfn_student::TFNStudentContract;
use tfn_launchpad::TFNLaunchpadContract;
use tfn_staking::TFNStakingContract;
use tfn_test_launchpad::TFNTestLaunchpadContract;
use tfn_test_staking::TFNTestStakingContract;
use tfn_test_dex::TFNTestDEXContract;
use tfn_nft_marketplace::TFNNFTMarketplaceContract;

use crate::{consts::*, exp18};

pub struct TFNContractSetup<
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
    pub blockchain_wrapper: BlockchainStateWrapper,
    pub owner: Address,
    pub dao_wrapper: ContractObjWrapper<tfn_dao::ContractObj<DebugApi>, TFNDAOContractObjBuilder>,
    pub dex_wrapper: ContractObjWrapper<tfn_dex::ContractObj<DebugApi>, TFNDEXContractObjBuilder>,
    pub platform_wrapper: ContractObjWrapper<tfn_platform::ContractObj<DebugApi>, TFNPlatformContractObjBuilder>,
    pub template_franchise_dao_wrapper: ContractObjWrapper<tfn_franchise_dao::ContractObj<DebugApi>, TFNFranchiseDAOContractObjBuilder>,
    pub template_employee_wrapper: ContractObjWrapper<tfn_employee::ContractObj<DebugApi>, TFNEmployeeContractObjBuilder>,
    pub template_student_wrapper: ContractObjWrapper<tfn_student::ContractObj<DebugApi>, TFNStudentContractObjBuilder>,
    pub launchpad_wrapper: ContractObjWrapper<tfn_launchpad::ContractObj<DebugApi>, TFNLaunchpadContractObjBuilder>,
    pub staking_wrapper: ContractObjWrapper<tfn_staking::ContractObj<DebugApi>, TFNStakingContractObjBuilder>,
    pub test_launchpad_wrapper: ContractObjWrapper<tfn_test_launchpad::ContractObj<DebugApi>, TFNTestLaunchpadContractObjBuilder>,
    pub test_staking_wrapper: ContractObjWrapper<tfn_test_staking::ContractObj<DebugApi>, TFNTestStakingContractObjBuilder>,
    pub test_dex_wrapper: ContractObjWrapper<tfn_test_dex::ContractObj<DebugApi>, TFNTestDEXContractObjBuilder>,
    pub nft_marketplace_wrapper: ContractObjWrapper<tfn_nft_marketplace::ContractObj<DebugApi>, TFNNFTMarketplaceContractObjBuilder>,
}

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
    pub fn new(
        dao_builder: TFNDAOContractObjBuilder,
        dex_builder: TFNDEXContractObjBuilder,
        platform_builder: TFNPlatformContractObjBuilder,
        franchise_dao_builder: TFNFranchiseDAOContractObjBuilder,
        employee_builder: TFNEmployeeContractObjBuilder,
        student_builder: TFNStudentContractObjBuilder,
        launchpad_builder: TFNLaunchpadContractObjBuilder,
        staking_builder: TFNStakingContractObjBuilder,
        test_launchpad_builder: TFNTestLaunchpadContractObjBuilder,
        test_staking_builder: TFNTestStakingContractObjBuilder,
        test_dex_builder: TFNTestDEXContractObjBuilder,
        nft_marketplace_builder: TFNNFTMarketplaceContractObjBuilder,
    ) -> Self {
        let mut blockchain_wrapper = BlockchainStateWrapper::new();
        let big_zero = rust_biguint!(0u64);
        let owner_address = blockchain_wrapper.create_user_account(&big_zero);
    
        // deploy DAO
        let dao_wrapper = blockchain_wrapper.create_sc_account(
            &big_zero,
            Some(&owner_address),
            dao_builder,
            DAO_WASM_PATH,
        );

        // init DAO
        blockchain_wrapper
            .execute_tx(&owner_address, &dao_wrapper, &big_zero, |sc| {
                sc.init(managed_token_id!(DAO_GOVERNANCE_TOKEN_ID))
            })
            .assert_ok();

        // set quorum
        blockchain_wrapper
            .execute_tx(&owner_address, &dao_wrapper, &big_zero, |sc| {
                sc.set_quorum(&managed_biguint!(DAO_QUORUM).pow(18));
            })
            .assert_ok();

        // set voting period
        blockchain_wrapper
            .execute_tx(&owner_address, &dao_wrapper, &big_zero, |sc| {
                sc.set_voting_period(DAO_VOTING_PERIOD);
            })
            .assert_ok();

        // deploy DEX
        let dex_wrapper = blockchain_wrapper.create_sc_account(
            &big_zero,
            Some(&owner_address),
            dex_builder,
            DEX_WASM_PATH,
        );

        // init DEX
        blockchain_wrapper
            .execute_tx(&owner_address, &dex_wrapper, &big_zero, |sc| {
                sc.init()
            })
            .assert_ok();

        // deploy platform
        let platform_wrapper = blockchain_wrapper.create_sc_account(
            &big_zero,
            Some(&owner_address),
            platform_builder,
            PLATFORM_WASM_PATH,
        );

        // init platform
        blockchain_wrapper
            .execute_tx(&owner_address, &platform_wrapper, &big_zero, |sc| {
                sc.init(managed_address!(dao_wrapper.address_ref()))
            })
            .assert_ok();
        
        // deploy franchise DAO
        let template_franchise_dao_wrapper = blockchain_wrapper.create_sc_account(
            &big_zero,
            Some(&owner_address),
            franchise_dao_builder,
            FRANCHISE_DAO_WASM_PATH,
        );

        // init franchise DAO
        blockchain_wrapper
            .execute_tx(&owner_address, &template_franchise_dao_wrapper, &big_zero, |sc| {
                sc.init(&managed_address!(&owner_address), &managed_token_id!(DAO_GOVERNANCE_TOKEN_ID))
            })
            .assert_ok();

        // deploy employee
        let template_employee_wrapper = blockchain_wrapper.create_sc_account(
            &big_zero,
            Some(&owner_address),
            employee_builder,
            EMPLOYEE_WASM_PATH,
        );

        // init employee
        blockchain_wrapper
            .execute_tx(&owner_address, &template_employee_wrapper, &big_zero, |sc| {
                sc.init("".into())
            })
            .assert_ok();

        // deploy student
        let template_student_wrapper = blockchain_wrapper.create_sc_account(
            &big_zero,
            Some(&owner_address),
            student_builder,
            STUDENT_WASM_PATH,
        );

        // init student
        blockchain_wrapper
            .execute_tx(&owner_address, &template_student_wrapper, &big_zero, |sc| {
                sc.init("".into())
            })
            .assert_ok();

        // deploy launchpad
        let launchpad_wrapper = blockchain_wrapper.create_sc_account(
            &big_zero,
            Some(&owner_address),
            launchpad_builder,
            LAUNCHPAD_WASM_PATH,
        );

        // init launchpad
        blockchain_wrapper
            .execute_tx(&owner_address, &launchpad_wrapper, &big_zero, |sc| {
                sc.init(
                    managed_address!(dao_wrapper.address_ref()),
                    managed_address!(dex_wrapper.address_ref()),
                    managed_address!(platform_wrapper.address_ref()),
                    managed_address!(template_franchise_dao_wrapper.address_ref()),
                    managed_address!(template_employee_wrapper.address_ref()),
                    managed_address!(template_student_wrapper.address_ref()),
                )
            })
            .assert_ok();

        // deploy staking
        let staking_wrapper = blockchain_wrapper.create_sc_account(
            &big_zero,
            Some(&owner_address),
            staking_builder,
            STAKING_WASM_PATH,
        );

        // init staking
        blockchain_wrapper
            .execute_tx(&owner_address, &staking_wrapper, &big_zero, |sc| {
                sc.init()
            })
            .assert_ok();

        // deploy test launchpad
        let test_launchpad_wrapper = blockchain_wrapper.create_sc_account(
            &big_zero,
            Some(&owner_address),
            test_launchpad_builder,
            TEST_LAUNCHPAD_WASM_PATH,
        );

        // init test launchpad
        blockchain_wrapper
            .execute_tx(&owner_address, &test_launchpad_wrapper, &big_zero, |sc| {
                sc.init(managed_address!(platform_wrapper.address_ref()))
            })
            .assert_ok();

        // deploy test staking
        let test_staking_wrapper = blockchain_wrapper.create_sc_account(
            &big_zero,
            Some(&owner_address),
            test_staking_builder,
            TEST_STAKING_WASM_PATH,
        );

        // init test staking
        blockchain_wrapper
            .execute_tx(&owner_address, &test_staking_wrapper, &big_zero, |sc| {
                sc.init(managed_address!(platform_wrapper.address_ref()))
            })
            .assert_ok();

        // deploy test DEX
        let test_dex_wrapper = blockchain_wrapper.create_sc_account(
            &big_zero,
            Some(&owner_address),
            test_dex_builder,
            TEST_DEX_WASM_PATH,
        );

        // init test DEX
        blockchain_wrapper
            .execute_tx(&owner_address, &test_dex_wrapper, &big_zero, |sc| {
                sc.init(managed_address!(platform_wrapper.address_ref()))
            })
            .assert_ok();

        // deploy NFT marketplace
        let nft_marketplace_wrapper = blockchain_wrapper.create_sc_account(
            &big_zero,
            Some(&owner_address),
            nft_marketplace_builder,
            NFT_MARKETPLACE_WASM_PATH,
        );

        // init NFT marketplace
        blockchain_wrapper
            .execute_tx(&owner_address, &nft_marketplace_wrapper, &big_zero, |sc| {
                sc.init(managed_address!(platform_wrapper.address_ref()))
            })
            .assert_ok();

        // activate DAO
        blockchain_wrapper
            .execute_tx(&owner_address, &dao_wrapper, &big_zero, |sc| {
                sc.set_launchpad_address(managed_address!(launchpad_wrapper.address_ref()));
            })
            .assert_ok();
        blockchain_wrapper
            .execute_tx(&owner_address, &dao_wrapper, &big_zero, |sc| {
                sc.set_state_active();
            })
            .assert_ok();

        // activate DEX
        blockchain_wrapper
            .execute_tx(&owner_address, &dex_wrapper, &big_zero, |sc| {
                sc.set_launchpad_address(managed_address!(launchpad_wrapper.address_ref()));
            })
            .assert_ok();
        blockchain_wrapper
            .execute_tx(&owner_address, &dex_wrapper, &big_zero, |sc| {
                sc.set_state_active();
            })
            .assert_ok();

        // activate platform
        blockchain_wrapper
            .execute_tx(&owner_address, &platform_wrapper, &big_zero, |sc| {
                sc.set_test_launchpad(managed_address!(test_launchpad_wrapper.address_ref()));
            })
            .assert_ok();
        blockchain_wrapper
            .execute_tx(&owner_address, &platform_wrapper, &big_zero, |sc| {
                sc.set_test_dex(managed_address!(test_dex_wrapper.address_ref()));
            })
            .assert_ok();
        blockchain_wrapper
            .execute_tx(&owner_address, &platform_wrapper, &big_zero, |sc| {
                sc.set_test_staking(managed_address!(test_staking_wrapper.address_ref()));
            })
            .assert_ok();
        blockchain_wrapper
            .execute_tx(&owner_address, &platform_wrapper, &big_zero, |sc| {
                sc.set_nft_marketplace(managed_address!(nft_marketplace_wrapper.address_ref()));
            })
            .assert_ok();
        blockchain_wrapper
            .execute_tx(&owner_address, &platform_wrapper, &big_zero, |sc| {
                sc.set_state_active();
            })
            .assert_ok();

        TFNContractSetup {
            blockchain_wrapper,
            owner: owner_address,
            dao_wrapper,
            dex_wrapper,
            platform_wrapper,
            template_franchise_dao_wrapper,
            template_employee_wrapper,
            template_student_wrapper,
            launchpad_wrapper,
            staking_wrapper,
            test_launchpad_wrapper,
            test_staking_wrapper,
            test_dex_wrapper,
            nft_marketplace_wrapper,
        }
    }

    pub fn setup_new_user(
        &mut self,
        egld_amount: u64
    ) -> Address {
        let big_zero = rust_biguint!(0);
        let new_user = self.blockchain_wrapper.create_user_account(&big_zero);
        
        self.blockchain_wrapper
            .set_egld_balance(&new_user, &exp18(egld_amount));
        
        new_user
    }
}
