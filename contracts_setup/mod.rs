#![allow(clippy::too_many_arguments)]

use multiversx_sc::types::Address;
use multiversx_sc_scenario::{
    managed_address, managed_buffer, managed_token_id, rust_biguint, testing_framework::*, DebugApi
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

use crate::{consts::*, contracts_interactions::common::exp18};

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
    
        // DEPLOYS

        // deploy DAO
        let dao_wrapper = blockchain_wrapper.create_sc_account(
            &big_zero,
            Some(&owner_address),
            dao_builder,
            DAO_WASM_PATH,
        );

        // deploy DEX
        let dex_wrapper = blockchain_wrapper.create_sc_account(
            &big_zero,
            Some(&owner_address),
            dex_builder,
            DEX_WASM_PATH,
        );

        // deploy template test launchpad
        let template_test_launchpad_wrapper = blockchain_wrapper.create_sc_account(
            &big_zero,
            Some(&owner_address),
            test_launchpad_builder,
            TEST_LAUNCHPAD_WASM_PATH,
        );

        // deploy template test staking
        let template_test_staking_wrapper = blockchain_wrapper.create_sc_account(
            &big_zero,
            Some(&owner_address),
            test_staking_builder,
            TEST_STAKING_WASM_PATH,
        );

        // deploy template test DEX
        let template_test_dex_wrapper = blockchain_wrapper.create_sc_account(
            &big_zero,
            Some(&owner_address),
            test_dex_builder,
            TEST_DEX_WASM_PATH,
        );

        // deploy template NFT marketplace
        let template_nft_marketplace_wrapper = blockchain_wrapper.create_sc_account(
            &big_zero,
            Some(&owner_address),
            nft_marketplace_builder,
            NFT_MARKETPLACE_WASM_PATH,
        );

        // deploy platform
        let platform_wrapper = blockchain_wrapper.create_sc_account(
            &big_zero,
            Some(&owner_address),
            platform_builder,
            PLATFORM_WASM_PATH,
        );

        // deploy franchise DAO
        let template_franchise_dao_wrapper = blockchain_wrapper.create_sc_account(
            &big_zero,
            Some(&owner_address),
            franchise_dao_builder,
            FRANCHISE_DAO_WASM_PATH,
        );

        // deploy employee
        let template_employee_wrapper = blockchain_wrapper.create_sc_account(
            &big_zero,
            Some(&owner_address),
            employee_builder,
            EMPLOYEE_WASM_PATH,
        );

        // deploy student
        let template_student_wrapper = blockchain_wrapper.create_sc_account(
            &big_zero,
            Some(&owner_address),
            student_builder,
            STUDENT_WASM_PATH,
        );

        // deploy launchpad
        let launchpad_wrapper = blockchain_wrapper.create_sc_account(
            &big_zero,
            Some(&owner_address),
            launchpad_builder,
            LAUNCHPAD_WASM_PATH,
        );

        // deploy staking
        let staking_wrapper = blockchain_wrapper.create_sc_account(
            &big_zero,
            Some(&owner_address),
            staking_builder,
            STAKING_WASM_PATH,
        );

        // INITS

        // init DAO
        blockchain_wrapper
            .execute_tx(&owner_address, &dao_wrapper, &big_zero, |sc| {
                sc.init(managed_token_id!(DAO_GOVERNANCE_TOKEN_ID))
            })
            .assert_ok();

        // set quorum
        blockchain_wrapper
            .execute_tx(&owner_address, &dao_wrapper, &big_zero, |sc| {
                sc.set_quorum(&exp18(DAO_QUORUM).sqrt().into());
            })
            .assert_ok();

        // set voting period
        blockchain_wrapper
            .execute_tx(&owner_address, &dao_wrapper, &big_zero, |sc| {
                sc.set_voting_period(DAO_VOTING_PERIOD);
            })
            .assert_ok();

        // init DEX
        blockchain_wrapper
            .execute_tx(&owner_address, &dex_wrapper, &big_zero, |sc| {
                sc.init()
            })
            .assert_ok();

        // init test launchpad
        blockchain_wrapper
            .execute_tx(&owner_address, &template_test_launchpad_wrapper, &big_zero, |sc| {
                sc.init()
            })
            .assert_ok();

        // init test staking
        blockchain_wrapper
            .execute_tx(&owner_address, &template_test_staking_wrapper, &big_zero, |sc| {
                sc.init()
            })
            .assert_ok();

        // init test DEX
        blockchain_wrapper
            .execute_tx(&owner_address, &template_test_dex_wrapper, &big_zero, |sc| {
                sc.init()
            })
            .assert_ok();

        // init NFT marketplace
        blockchain_wrapper
            .execute_tx(&owner_address, &template_nft_marketplace_wrapper, &big_zero, |sc| {
                sc.init()
            })
            .assert_ok();

        // init platform
        blockchain_wrapper
            .execute_tx(&owner_address, &platform_wrapper, &big_zero, |sc| {
                sc.init(
                    managed_address!(template_test_launchpad_wrapper.address_ref()),
                    managed_address!(template_test_dex_wrapper.address_ref()),
                    managed_address!(template_test_staking_wrapper.address_ref()),
                    managed_address!(template_nft_marketplace_wrapper.address_ref()),
                )
            })
            .assert_ok();
        
        // init franchise DAO
        blockchain_wrapper
            .execute_tx(&owner_address, &template_franchise_dao_wrapper, &big_zero, |sc| {
                sc.init(&managed_address!(&owner_address), &managed_token_id!(FRANCHISE1_GOVERNANCE_TOKEN_ID))
            })
            .assert_ok();

        // init employee
        blockchain_wrapper
            .execute_tx(&owner_address, &template_employee_wrapper, &big_zero, |sc| {
                sc.init("".into())
            })
            .assert_ok();

        // init student
        blockchain_wrapper
            .execute_tx(&owner_address, &template_student_wrapper, &big_zero, |sc| {
                sc.init("".into())
            })
            .assert_ok();

        // init launchpad
        blockchain_wrapper
            .execute_tx(&owner_address, &launchpad_wrapper, &big_zero, |sc| {
                sc.init(
                    managed_address!(dao_wrapper.address_ref()),
                    managed_address!(dex_wrapper.address_ref()),
                )
            })
            .assert_ok();

        // init staking
        blockchain_wrapper
            .execute_tx(&owner_address, &staking_wrapper, &big_zero, |sc| {
                sc.init()
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
                sc.set_platform_address(managed_address!(platform_wrapper.address_ref()));
            })
            .assert_ok();
        blockchain_wrapper
            .execute_tx(&owner_address, &dao_wrapper, &big_zero, |sc| {
                sc.set_template_addresses(
                    managed_address!(template_franchise_dao_wrapper.address_ref()),
                    managed_address!(template_employee_wrapper.address_ref()),
                    managed_address!(template_student_wrapper.address_ref()),
                );
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
                sc.set_main_dao(managed_address!(dao_wrapper.address_ref()));
            })
            .assert_ok();
        blockchain_wrapper
            .execute_tx(&owner_address, &platform_wrapper, &big_zero, |sc| {
                sc.set_state_active();
            })
            .assert_ok();

        // subscribe on platform
        let mut nft_marketplace_address= Address::zero();
        let mut test_launchpad_address= Address::zero();
        let mut test_dex_address= Address::zero();
        let mut test_staking_address= Address::zero();
        blockchain_wrapper
            .execute_tx(launchpad_wrapper.address_ref(), &platform_wrapper, &big_zero, |sc| {
                sc.subscribe_franchise(managed_address!(&owner_address), tfn_platform::common::config::SubscriberDetails {
                    name: managed_buffer!(b""),
                    description: managed_buffer!(b""),
                    logo: managed_buffer!(b""),
                    card: managed_buffer!(b""),
                    website: managed_buffer!(b""),
                    email: managed_buffer!(b""),
                    twitter: managed_buffer!(b""),
                    telegram: managed_buffer!(b""),
                });
            })
            .assert_ok();

        blockchain_wrapper
            .execute_query(&platform_wrapper, |sc| {
                let subscriber_id = sc.get_subscriber_id_by_address(&managed_address!(&owner_address));
                let subscriber = sc.subscribers(subscriber_id.unwrap()).get();
                nft_marketplace_address = subscriber.nft_marketplace_sc.to_address();
                test_launchpad_address = subscriber.launchpad_sc.to_address();
                test_dex_address = subscriber.dex_sc.to_address();
                test_staking_address = subscriber.staking_sc.to_address();
            })
            .assert_ok();

        // deploy NFT Marketplace
        let nft_marketplace_wrapper = blockchain_wrapper.create_sc_account_fixed_address(
            &nft_marketplace_address,
            &big_zero,
            Some(&owner_address),
            nft_marketplace_builder,
            NFT_MARKETPLACE_WASM_PATH,
        );

        // deploy test Launchpad
        let test_launchpad_wrapper = blockchain_wrapper.create_sc_account_fixed_address(
            &test_launchpad_address,
            &big_zero,
            Some(&owner_address),
            test_launchpad_builder,
            TEST_LAUNCHPAD_WASM_PATH,
        );

        // deploy test DEX
        let test_dex_wrapper = blockchain_wrapper.create_sc_account_fixed_address(
            &test_dex_address,
            &big_zero,
            Some(&owner_address),
            test_dex_builder,
            TEST_DEX_WASM_PATH,
        );

        // deploy test Staking
        let test_staking_wrapper = blockchain_wrapper.create_sc_account_fixed_address(
            &test_staking_address,
            &big_zero,
            Some(&owner_address),
            test_staking_builder,
            TEST_STAKING_WASM_PATH,
        );

        // init NFT Marketplace
        blockchain_wrapper
            .execute_tx(platform_wrapper.address_ref(), &nft_marketplace_wrapper, &big_zero, |sc| {
                sc.init()
            })
            .assert_ok();

        // init test Launchpad
        blockchain_wrapper
            .execute_tx(platform_wrapper.address_ref(), &test_launchpad_wrapper, &big_zero, |sc| {
                sc.init()
            })
            .assert_ok();

        // init test DEX
        blockchain_wrapper
            .execute_tx(platform_wrapper.address_ref(), &test_dex_wrapper, &big_zero, |sc| {
                sc.init()
            })
            .assert_ok();

        // init test Staking
        blockchain_wrapper
            .execute_tx(platform_wrapper.address_ref(), &test_staking_wrapper, &big_zero, |sc| {
                sc.init()
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
}
