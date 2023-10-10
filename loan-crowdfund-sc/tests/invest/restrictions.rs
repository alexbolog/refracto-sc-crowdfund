use loan_crowdfund_sc::{
    constants::{ERR_CANNOT_INVEST_IN_CRT_STATE, ERR_CANNOT_OVER_FINANCE},
    types::crowdfunding_state::ProjectFundingState,
};

use crate::test_state::{LoanCfTestState, INVESTOR_1_ADDRESS_EXPR};

#[test]
fn failed_invest_during_funding_state_pending() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_fully_mocked_project();

    state.check_funding_state(1, ProjectFundingState::Pending);
    state.invest_and_expect_err(
        INVESTOR_1_ADDRESS_EXPR,
        100,
        1,
        ERR_CANNOT_INVEST_IN_CRT_STATE,
    );
}

#[test]
fn failed_invest_during_funding_state_successful() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_fully_mocked_project();
    state.set_block_timestamp(101);
    state.invest(INVESTOR_1_ADDRESS_EXPR, 9001, 1);
    state.set_block_timestamp(10001);

    state.check_funding_state(1, ProjectFundingState::CFSuccessful);

    state.invest_and_expect_err(
        INVESTOR_1_ADDRESS_EXPR,
        100,
        1,
        ERR_CANNOT_INVEST_IN_CRT_STATE,
    );
}

#[test]
fn failed_invest_during_funding_state_failed() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_fully_mocked_project();
    state.set_block_timestamp(10001);

    state.check_funding_state(1, ProjectFundingState::CFFailed);

    state.invest_and_expect_err(
        INVESTOR_1_ADDRESS_EXPR,
        100,
        1,
        ERR_CANNOT_INVEST_IN_CRT_STATE,
    );
}

#[test]
fn failed_invest_during_funding_state_cancelled() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_fully_mocked_project();
    state.cancel_project(1);

    state.invest_and_expect_err(
        INVESTOR_1_ADDRESS_EXPR,
        100,
        1,
        ERR_CANNOT_INVEST_IN_CRT_STATE,
    );
}

#[test]
fn failed_invest_during_funding_state_loan_active() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_fully_mocked_project();
    state.set_block_timestamp(101);
    state.invest(INVESTOR_1_ADDRESS_EXPR, 9001, 1);
    state.set_block_timestamp(10001);
    state.claim(INVESTOR_1_ADDRESS_EXPR, 1, 1);

    state.check_funding_state(1, ProjectFundingState::LoanActive);

    state.invest_and_expect_err(
        INVESTOR_1_ADDRESS_EXPR,
        100,
        1,
        ERR_CANNOT_INVEST_IN_CRT_STATE,
    );
}

#[test]
fn failed_invest_during_funding_state_completed() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_fully_mocked_project();
    state.set_block_timestamp(101);
    state.invest(INVESTOR_1_ADDRESS_EXPR, 9001, 1);
    state.set_block_timestamp(10001);
    state.claim_loan_funds(1);

    state.repay_loan(1, 11000);
    state.admin_distribute_repayment(1);

    state.check_funding_state(1, ProjectFundingState::Completed);

    state.invest_and_expect_err(
        INVESTOR_1_ADDRESS_EXPR,
        100,
        1,
        ERR_CANNOT_INVEST_IN_CRT_STATE,
    );
}

#[test]
fn failed_over_financing() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_fully_mocked_project();
    state.set_block_timestamp(101);
    state.invest(INVESTOR_1_ADDRESS_EXPR, 99999, 1);

    state.invest_and_expect_err(INVESTOR_1_ADDRESS_EXPR, 100, 1, ERR_CANNOT_OVER_FINANCE)
}
