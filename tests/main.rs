use loan_crowdfund_sc::types::crowdfunding_state::ProjectFundingState;
use test_state::{
    LoanCfTestState, ACCOUNT_BALANCE_EXPR, INVESTOR_1_ADDRESS_EXPR, LOAN_SHARES_ID, USDC_TOKEN_ID,
};

mod invest;
mod project_funding_state;
mod test_state;

#[test]
fn test_deploy() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
}

#[test]
fn test_create_project() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();

    state.create_valid_mockup_project();

    state.check_funding_state(1, ProjectFundingState::Pending);
}

#[test]
fn test_invest() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();

    state.create_valid_mockup_project();

    state.invest(INVESTOR_1_ADDRESS_EXPR, 100, 1);
    state.check_investor_share_balance(INVESTOR_1_ADDRESS_EXPR, "1", "1");
}

#[test]
fn test_cancel_project() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();

    state.create_valid_mockup_project();

    state.cancel_project(1);
}

#[test]
fn claim_cancelled_project_funds() {
    let mut state = LoanCfTestState::new();
    state.deploy_contract();
    state.create_valid_mockup_project();
    state.invest(INVESTOR_1_ADDRESS_EXPR, 100, 1);
    state.cancel_project(1);

    state.claim(INVESTOR_1_ADDRESS_EXPR, 1, 1);

    state.check_investor_usdc_balance(INVESTOR_1_ADDRESS_EXPR, ACCOUNT_BALANCE_EXPR);
    state.check_investor_share_balance(INVESTOR_1_ADDRESS_EXPR, "1", "0");
}
