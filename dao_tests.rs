use multiversx_sc_scenario::{managed_address, rust_biguint, DebugApi};
use tfn_dao::{multisig::MultisigModule, TFNDAOContract};

use crate::contracts_setup::TFNContractSetup;

#[test]
fn dao_add_new_board_member_test() {
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
    let new_board_member = sc_setup.setup_new_user(1u64);
    let mut action_id: usize = 0;
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            action_id = sc.propose_add_board_member(managed_address!(&new_board_member));
        })
        .assert_ok();
    sc_setup.blockchain_wrapper
        .execute_tx(&sc_setup.owner, &sc_setup.dao_wrapper, &big_zero, |sc| {
            sc.perform_action(action_id);
        })
        .assert_ok();
}
