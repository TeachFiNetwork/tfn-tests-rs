mod consts;
mod contracts_setup;
mod dao_tests;

use multiversx_sc_scenario::{rust_biguint, num_bigint, DebugApi};

use crate::contracts_setup::TFNContractSetup;

use std::ops::Mul;

#[test]
fn init_test() {
    DebugApi::dummy();
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
}

pub fn err2str(err: &[u8]) -> &str {
    &(std::str::from_utf8(err).unwrap())
}

pub fn exp18(value: u64) -> num_bigint::BigUint {
    value.mul(rust_biguint!(10).pow(18))
}
