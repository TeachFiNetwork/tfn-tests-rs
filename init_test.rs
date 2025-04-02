use multiversx_sc_scenario::DebugApi;
use crate::contracts_setup::TFNContractSetup;

#[test]
fn init_test() {
    DebugApi::dummy();
    let _ = TFNContractSetup::new(
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
}
