use loan_crowdfund_sc::constants::{
    ERR_CANNOT_INVEST_IN_CRT_STATE, ERR_INVALID_PAYMENT_TOKEN, ERR_KYC_NOT_DONE,
};

use crate::test_state::{LoanCfTestState, INVALID_TOKEN_ID_EXPR, INVESTOR_1_ADDRESS_EXPR};

#[test]
fn invest_fails_without_kyc_whitelist() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_fully_mocked_project();

    state.invest_and_expect_err(INVESTOR_1_ADDRESS_EXPR, 1000, 1, ERR_KYC_NOT_DONE);
}

#[test]
fn invest_succeeds_with_kyc_whitelist() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_fully_mocked_project();
    state.whitelist_address(INVESTOR_1_ADDRESS_EXPR);

    state.invest(INVESTOR_1_ADDRESS_EXPR, 1000, 1);
}

#[test]
fn invest_fails_with_invalid_project_id() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_fully_mocked_project();
    state.whitelist_address(INVESTOR_1_ADDRESS_EXPR);

    state.explicit_invest_and_expect_err(
        INVESTOR_1_ADDRESS_EXPR,
        INVALID_TOKEN_ID_EXPR,
        1000,
        2,
        ERR_INVALID_PAYMENT_TOKEN,
    );
}

#[test]
fn invest_fails_if_state_not_active() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_fully_mocked_project();
    state.whitelist_address(INVESTOR_1_ADDRESS_EXPR);

    state.invest(INVESTOR_1_ADDRESS_EXPR, 1000, 1);

    state.invest_and_expect_err(
        INVESTOR_1_ADDRESS_EXPR,
        1000,
        1,
        ERR_CANNOT_INVEST_IN_CRT_STATE,
    );
}
