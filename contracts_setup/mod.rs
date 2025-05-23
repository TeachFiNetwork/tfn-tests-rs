#![allow(clippy::too_many_arguments)]

use multiversx_sc::types::{Address, ManagedVec};
use multiversx_sc_scenario::{
    managed_address, managed_biguint, managed_buffer, managed_token_id, rust_biguint, testing_framework::*, DebugApi
};

use tfn_dao::{common::config::{ConfigModule as _, LaunchpadProposal}, TFNDAOContract};
use tfn_dex::{common::consts::TOKEN_ISSUE_COST, TFNDEXContract};
use tfn_platform::{common::config::ConfigModule as _, TFNPlatformContract};
use tfn_franchise_dao::{common::config::ConfigModule, school::SchoolModule, TFNFranchiseDAOContract};
use tfn_launchpad::{TFNLaunchpadContract, common::config::ConfigModule as _};
use tfn_staking::TFNStakingContract;
use tfn_test_launchpad::TFNTestLaunchpadContract;
use tfn_test_staking::TFNTestStakingContract;
use tfn_test_dex::TFNTestDEXContract;
use tfn_nft_marketplace::TFNNFTMarketplaceContract;
use tfn_digital_identity::{common::config::Identity, TFNDigitalIdentityContract};

use crate::{consts::*, contracts_interactions::common::exp18};

pub struct TFNContractSetup<
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
    pub blockchain_wrapper: BlockchainStateWrapper,
    pub owner: Address,
    pub dao_wrapper: ContractObjWrapper<tfn_dao::ContractObj<DebugApi>, TFNDAOContractObjBuilder>,
    pub dex_wrapper: ContractObjWrapper<tfn_dex::ContractObj<DebugApi>, TFNDEXContractObjBuilder>,
    pub platform_wrapper: ContractObjWrapper<tfn_platform::ContractObj<DebugApi>, TFNPlatformContractObjBuilder>,
    pub franchise_dao_wrapper: ContractObjWrapper<tfn_franchise_dao::ContractObj<DebugApi>, TFNFranchiseDAOContractObjBuilder>,
    pub launchpad_wrapper: ContractObjWrapper<tfn_launchpad::ContractObj<DebugApi>, TFNLaunchpadContractObjBuilder>,
    pub staking_wrapper: ContractObjWrapper<tfn_staking::ContractObj<DebugApi>, TFNStakingContractObjBuilder>,
    pub test_launchpad_wrapper: ContractObjWrapper<tfn_test_launchpad::ContractObj<DebugApi>, TFNTestLaunchpadContractObjBuilder>,
    pub test_staking_wrapper: ContractObjWrapper<tfn_test_staking::ContractObj<DebugApi>, TFNTestStakingContractObjBuilder>,
    pub test_dex_wrapper: ContractObjWrapper<tfn_test_dex::ContractObj<DebugApi>, TFNTestDEXContractObjBuilder>,
    pub nft_marketplace_wrapper: ContractObjWrapper<tfn_nft_marketplace::ContractObj<DebugApi>, TFNNFTMarketplaceContractObjBuilder>,
    pub digital_identity_wrapper: ContractObjWrapper<tfn_digital_identity::ContractObj<DebugApi>, TFNDigitalIdentityContractObjBuilder>,
}

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
    pub fn new(
        dao_builder: TFNDAOContractObjBuilder,
        dex_builder: TFNDEXContractObjBuilder,
        platform_builder: TFNPlatformContractObjBuilder,
        franchise_dao_builder: TFNFranchiseDAOContractObjBuilder,
        launchpad_builder: TFNLaunchpadContractObjBuilder,
        staking_builder: TFNStakingContractObjBuilder,
        test_launchpad_builder: TFNTestLaunchpadContractObjBuilder,
        test_staking_builder: TFNTestStakingContractObjBuilder,
        test_dex_builder: TFNTestDEXContractObjBuilder,
        nft_marketplace_builder: TFNNFTMarketplaceContractObjBuilder,
        digital_identity_builder: TFNDigitalIdentityContractObjBuilder,
    ) -> Self {
        let mut blockchain_wrapper = BlockchainStateWrapper::new();
        let big_zero = rust_biguint!(0u64);
        let owner_address = blockchain_wrapper.create_user_account(&rust_biguint!(TOKEN_ISSUE_COST));
    
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

        // deploy digital identity
        let digital_identity_wrapper = blockchain_wrapper.create_sc_account(
            &big_zero,
            Some(&owner_address),
            digital_identity_builder,
            DIGITAL_IDENTITY_WASM_PATH,
        );

        // INITS

        // init DAO
        blockchain_wrapper
            .execute_tx(&owner_address, &dao_wrapper, &big_zero, |sc| {
                sc.init(managed_token_id!(DAO_GOVERNANCE_TOKEN_ID))
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
        
        // init template franchise DAO
        blockchain_wrapper
            .execute_tx(&owner_address, &template_franchise_dao_wrapper, &big_zero, |sc| {
                sc.init(
                    &managed_address!(&owner_address),
                    &managed_token_id!(FRANCHISE1_GOVERNANCE_TOKEN_ID),
                )
            })
            .assert_ok();

        // init launchpad
        blockchain_wrapper
            .execute_tx(&owner_address, &launchpad_wrapper, &big_zero, |sc| {
                sc.init()
            })
            .assert_ok();

        // init staking
        blockchain_wrapper
            .execute_tx(&owner_address, &staking_wrapper, &big_zero, |sc| {
                sc.init()
            })
            .assert_ok();

        // init digital identity
        blockchain_wrapper
            .execute_tx(&owner_address, &digital_identity_wrapper, &big_zero, |sc| {
                sc.init()
            })
            .assert_ok();

        // ACTIVATIONS

        // activate DAO
        blockchain_wrapper.execute_tx(&owner_address, &dao_wrapper, &big_zero, |sc| {
            sc.set_quorum(&exp18(DAO_QUORUM).sqrt().into());
            sc.set_voting_period(DAO_VOTING_PERIOD);
            sc.set_launchpad_address(managed_address!(launchpad_wrapper.address_ref()));
            sc.set_platform_address(managed_address!(platform_wrapper.address_ref()));
            sc.set_template_franchise_dao(managed_address!(template_franchise_dao_wrapper.address_ref()));
            sc.set_digital_identity_address(managed_address!(digital_identity_wrapper.address_ref()));
            sc.set_state_active();
        })
        .assert_ok();

        // activate platform
        blockchain_wrapper.execute_tx(&owner_address, &platform_wrapper, &big_zero, |sc| {
            sc.set_state_active();
        })
        .assert_ok();

        // activate launchpad
        blockchain_wrapper.execute_tx(&owner_address, &launchpad_wrapper, &big_zero, |sc| {
            sc.set_dex(managed_address!(dex_wrapper.address_ref()));
            sc.set_state_active();
        })
        .assert_ok();

        // LAUNCH A FRANCHISE

        let mut franchise_address = Address::zero();
        let mut nft_marketplace_address= Address::zero();
        let mut test_launchpad_address= Address::zero();
        let mut test_dex_address= Address::zero();
        let mut test_staking_address= Address::zero();
        let mut proposal_id = 0;
        let mut launchpad_id = 0;
        let launchpad_duration = 1;
        let vote_amount = exp18(DAO_QUORUM);
        blockchain_wrapper.set_esdt_balance(&owner_address, DAO_GOVERNANCE_TOKEN_ID.as_bytes(), &vote_amount);
        blockchain_wrapper.execute_tx(&owner_address, &dao_wrapper, &big_zero, |sc| {
            proposal_id = sc.propose_new_launchpad(
                managed_buffer!(b"title"),
                managed_buffer!(b"description"),
                LaunchpadProposal{
                    details: Identity {
                        id: 0,
                        is_corporate: false,
                        legal_id: managed_biguint!(0),
                        birthdate: 0,
                        address: managed_address!(&Address::zero()),
                        name: managed_buffer!(SCHOOL_NAME.as_bytes()),
                        description: managed_buffer!(b""),
                        image: managed_buffer!(b""),
                        contact: ManagedVec::new(),
                    },
                    kyc_enforced: false,
                    token: managed_token_id!(FRANCHISE0_GOVERNANCE_TOKEN_ID),
                    payment_token: managed_token_id!(DAO_GOVERNANCE_TOKEN_ID),
                    price: managed_biguint!(1_000_000_000_000_000_000),
                    min_buy_amount: managed_biguint!(1_000_000_000_000_000_000),
                    max_buy_amount: managed_biguint!(1_000_000_000_000_000_000),
                    start_time: DAO_VOTING_PERIOD + 1,
                    end_time: DAO_VOTING_PERIOD + 1 + launchpad_duration,
                },
            );
        })
        .assert_ok();
        blockchain_wrapper.execute_esdt_transfer(
            &owner_address,
            &dao_wrapper,
            DAO_GOVERNANCE_TOKEN_ID.as_bytes(),
            0,
            &vote_amount,
            |sc| {
                sc.upvote(proposal_id);
            },
        )
        .assert_ok();
        blockchain_wrapper.set_block_timestamp(DAO_VOTING_PERIOD);
        blockchain_wrapper.execute_tx(&owner_address, &dao_wrapper, &rust_biguint!(0u64), |sc| {
            sc.execute(proposal_id);
        })
        .assert_ok();
        blockchain_wrapper.set_block_timestamp(DAO_VOTING_PERIOD + 1 + launchpad_duration + 1);
        blockchain_wrapper.execute_tx(&owner_address, &launchpad_wrapper, &rust_biguint!(TOKEN_ISSUE_COST), |sc| {
            launchpad_id = sc.last_launchpad_id().get() - 1;
            franchise_address = sc.deploy_franchise(launchpad_id).to_address();
        })
        .assert_ok();
        blockchain_wrapper.execute_query(&platform_wrapper, |sc| {
            let (opt_subscriber, _) = sc.get_address_details(managed_address!(&franchise_address));
            let subscriber = opt_subscriber.unwrap();
            test_launchpad_address = subscriber.launchpad_sc.to_address();
            test_dex_address = subscriber.dex_sc.to_address();
            test_staking_address = subscriber.staking_sc.to_address();
            nft_marketplace_address = subscriber.nft_marketplace_sc.to_address();
        })
        .assert_ok();

        // deploy franchise DAO
        let franchise_dao_wrapper = blockchain_wrapper.create_sc_account_fixed_address(
            &franchise_address,
            &big_zero,
            Some(launchpad_wrapper.address_ref()),
            franchise_dao_builder,
            FRANCHISE_DAO_WASM_PATH,
        );
        // create student and teacher identities
        let student_address = Address::from_slice(STUDENT_ADDRESS.as_bytes());
        let employee_address = Address::from_slice(TEACHER_ADDRESS.as_bytes());
        blockchain_wrapper.create_user_account_fixed_address(&student_address, &big_zero);
        blockchain_wrapper.create_user_account_fixed_address(&employee_address, &big_zero);
        let mut student_identity_id = 0;
        let mut teacher_identity_id = 0;
        blockchain_wrapper.execute_tx(&owner_address, &digital_identity_wrapper, &big_zero, |sc| {
            student_identity_id = sc.new_identity(
                false,
                managed_biguint!(0),
                0,
                managed_address!(&student_address),
                managed_buffer!(STUDENT_NAME.as_bytes()),
                managed_buffer!(b""),
                managed_buffer!(b""),
                ManagedVec::new(),
            );
            teacher_identity_id = sc.new_identity(
                false,
                managed_biguint!(0),
                0,
                managed_address!(&employee_address),
                managed_buffer!(TEACHER_NAME.as_bytes()),
                managed_buffer!(b""),
                managed_buffer!(b""),
                ManagedVec::new(),
            );
        })
        .assert_ok();
        // init franchise DAO
        blockchain_wrapper.execute_tx(launchpad_wrapper.address_ref(), &franchise_dao_wrapper, &big_zero, |sc| {
            sc.init(
                &managed_address!(&owner_address),
                &managed_token_id!(FRANCHISE0_GOVERNANCE_TOKEN_ID),
            );
        })
        .assert_ok();

        let mut teacher_link_request_id = 0;
        let mut student_link_request_id = 0;
        blockchain_wrapper.execute_tx(&owner_address, &franchise_dao_wrapper, &big_zero, |sc| {
            sc.set_quorum(&exp18(DAO_QUORUM).sqrt().into());
            sc.set_voting_period(DAO_VOTING_PERIOD);
            sc.set_state_active();
            teacher_link_request_id = sc.register_employee_identity(teacher_identity_id, true);
            student_link_request_id = sc.register_student_identity(student_identity_id);
        })
        .assert_ok();
        blockchain_wrapper.execute_tx(&employee_address, &digital_identity_wrapper, &big_zero, |sc| {
            sc.accept_link(teacher_link_request_id);
        })
        .assert_ok();
        blockchain_wrapper.execute_tx(&student_address, &digital_identity_wrapper, &big_zero, |sc| {
            sc.accept_link(student_link_request_id);
        })
        .assert_ok();
        blockchain_wrapper.execute_tx(&owner_address, &franchise_dao_wrapper, &big_zero, |sc| {
            sc.hire_employee(teacher_identity_id, managed_buffer!(b"teacher"), managed_biguint!(0));
            let class_id = sc.create_class(2025, managed_buffer!(CLASS_NAME.as_bytes()));
            sc.enroll_student(student_identity_id, class_id);
        })
        .assert_ok();

        // deploy NFT Marketplace
        let nft_marketplace_wrapper = blockchain_wrapper.create_sc_account_fixed_address(
            &nft_marketplace_address,
            &big_zero,
            Some(platform_wrapper.address_ref()),
            nft_marketplace_builder,
            NFT_MARKETPLACE_WASM_PATH,
        );

        // deploy test Launchpad
        let test_launchpad_wrapper = blockchain_wrapper.create_sc_account_fixed_address(
            &test_launchpad_address,
            &big_zero,
            Some(platform_wrapper.address_ref()),
            test_launchpad_builder,
            TEST_LAUNCHPAD_WASM_PATH,
        );

        // deploy test DEX
        let test_dex_wrapper = blockchain_wrapper.create_sc_account_fixed_address(
            &test_dex_address,
            &big_zero,
            Some(platform_wrapper.address_ref()),
            test_dex_builder,
            TEST_DEX_WASM_PATH,
        );

        // deploy test Staking
        let test_staking_wrapper = blockchain_wrapper.create_sc_account_fixed_address(
            &test_staking_address,
            &big_zero,
            Some(platform_wrapper.address_ref()),
            test_staking_builder,
            TEST_STAKING_WASM_PATH,
        );

        // init NFT Marketplace
        blockchain_wrapper.execute_tx(platform_wrapper.address_ref(), &nft_marketplace_wrapper, &big_zero, |sc| {
            sc.init()
        })
        .assert_ok();

        // init test Launchpad
        blockchain_wrapper.execute_tx(platform_wrapper.address_ref(), &test_launchpad_wrapper, &big_zero, |sc| {
            sc.init()
        })
        .assert_ok();

        // init test DEX
        blockchain_wrapper.execute_tx(platform_wrapper.address_ref(), &test_dex_wrapper, &big_zero, |sc| {
            sc.init()
        })
        .assert_ok();

        // init test Staking
        blockchain_wrapper.execute_tx(platform_wrapper.address_ref(), &test_staking_wrapper, &big_zero, |sc| {
            sc.init()
        })
        .assert_ok();

        blockchain_wrapper.set_block_timestamp(0);

        TFNContractSetup {
            blockchain_wrapper,
            owner: owner_address,
            dao_wrapper,
            dex_wrapper,
            platform_wrapper,
            franchise_dao_wrapper,
            launchpad_wrapper,
            staking_wrapper,
            test_launchpad_wrapper,
            test_staking_wrapper,
            test_dex_wrapper,
            nft_marketplace_wrapper,
            digital_identity_wrapper,
        }
    }
}
