use crate::test_state::{
    mockups::{
        MOCKUP_CF_DEFAULT_COVER_MIN_PRINCIPAL, MOCKUP_CF_TIMESTAMP_AFTER_END,
        MOCKUP_CF_TIMESTAMP_AFTER_START, MOCKUP_CF_TIMESTAMP_BEFORE_END,
    },
    LoanCfTestState, INVESTOR_1_ADDRESS_EXPR,
};
use loan_crowdfund_sc::types::crowdfunding_state::ProjectFundingState;

#[test]
fn funding_state_pending() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_fully_mocked_project();

    state.check_funding_state(1, ProjectFundingState::Pending);
}

#[test]
fn funding_state_active() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_fully_mocked_project();
    state.set_block_timestamp(MOCKUP_CF_TIMESTAMP_AFTER_START);

    state.check_funding_state(1, ProjectFundingState::CFActive);
}

#[test]
fn funding_state_cool_off() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_fully_mocked_project();
    state.whitelist_address(INVESTOR_1_ADDRESS_EXPR);

    state.set_block_timestamp(MOCKUP_CF_TIMESTAMP_BEFORE_END);
    state.invest(
        INVESTOR_1_ADDRESS_EXPR,
        MOCKUP_CF_DEFAULT_COVER_MIN_PRINCIPAL,
        1,
    );
    state.set_block_timestamp(MOCKUP_CF_TIMESTAMP_AFTER_END);
    state.check_funding_state(1, ProjectFundingState::CFWaitingCooloff);
}

#[test]
fn funding_state_successful() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_fully_mocked_project();
    state.whitelist_address(INVESTOR_1_ADDRESS_EXPR);

    state.set_block_timestamp(MOCKUP_CF_TIMESTAMP_AFTER_START);
    state.invest(
        INVESTOR_1_ADDRESS_EXPR,
        MOCKUP_CF_DEFAULT_COVER_MIN_PRINCIPAL,
        1,
    );
    state.set_block_timestamp(MOCKUP_CF_TIMESTAMP_AFTER_END);

    state.check_funding_state(1, ProjectFundingState::CFSuccessful);
}

#[test]
fn funding_state_failed() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_fully_mocked_project();
    state.set_block_timestamp(MOCKUP_CF_TIMESTAMP_AFTER_END);

    state.check_funding_state(1, ProjectFundingState::CFFailed);
}

#[test]
fn funding_state_cancelled() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_fully_mocked_project();
    state.cancel_project(1);

    state.check_funding_state(1, ProjectFundingState::CFCancelled);
}

#[test]
fn funding_state_loan_active() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_fully_mocked_project();
    state.whitelist_address(INVESTOR_1_ADDRESS_EXPR);

    state.set_block_timestamp(MOCKUP_CF_TIMESTAMP_AFTER_START);
    state.invest(
        INVESTOR_1_ADDRESS_EXPR,
        MOCKUP_CF_DEFAULT_COVER_MIN_PRINCIPAL,
        1,
    );
    state.set_block_timestamp(MOCKUP_CF_TIMESTAMP_AFTER_END);

    state.claim_loan_funds(1);
    state.check_funding_state(1, ProjectFundingState::LoanActive);
}

#[test]
fn funding_state_completed() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_fully_mocked_project();
    state.whitelist_address(INVESTOR_1_ADDRESS_EXPR);

    state.set_block_timestamp(MOCKUP_CF_TIMESTAMP_AFTER_START);
    state.invest(
        INVESTOR_1_ADDRESS_EXPR,
        MOCKUP_CF_DEFAULT_COVER_MIN_PRINCIPAL,
        1,
    );
    state.set_block_timestamp(MOCKUP_CF_TIMESTAMP_AFTER_END);
    state.claim_loan_funds(1);

    state.repay_loan(1, 11000);
    state.admin_distribute_repayment(1);

    state.check_funding_state(1, ProjectFundingState::Completed);
}
